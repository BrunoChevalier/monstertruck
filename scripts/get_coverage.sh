#!/usr/bin/env bash
SCRIPT_DIR=$(realpath "${BASH_SOURCE[0]%/*}")
APP_DIR=$(realpath "${SCRIPT_DIR}/..")
PROFILING_DATA_DIR="${APP_DIR}/profiling_data"
MERGED_PROFDATA_FILE="$PROFILING_DATA_DIR/all_merged_profdata"

# Coverage generation script for rust apps

cd "$APP_DIR"

# Function to check for required tools
check_tools() {
    all_tools_present="yes"
    if ! type cargo >/dev/null 2>&1; then
        echo "Error: cargo not found in PATH"
        all_tools_present="no"
    fi

    if ! type llvm-profdata >/dev/null 2>&1; then
        echo "Error: llvm-profdata not found in PATH"
        echo "Please install LLVM coverage tools. If using rustup:"
        echo "  rustup component add llvm-tools-preview"
        echo "Then ensure the tools are in PATH or install llvm package."
        all_tools_present="no"
    fi

    if ! type llvm-cov >/dev/null 2>&1; then
        echo "Error: llvm-cov not found in PATH"
        echo "Please install LLVM coverage tools. If using rustup:"
        echo "  rustup component add llvm-tools-preview"
        echo "Then ensure the tools are in PATH or install llvm package."
        all_tools_present="no"
    fi

    if [ "${all_tools_present}" != "yes" ]; then
        exit 1
    fi
}

# Function to clean existing coverage files
clean_coverage_files() {
    echo "Cleaning coverage files..."
    rm -f ./*.profraw ./*.profdata
    if [ -d "$PROFILING_DATA_DIR" ]; then
        rm -rf "$PROFILING_DATA_DIR"
    fi
    echo "Coverage files cleaned."
}

# Function to generate profile data
generate_profile_data() {
    mkdir -p "$PROFILING_DATA_DIR"

    echo "Building with coverage instrumentation..."
    env RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="$PROFILING_DATA_DIR/build-%p-%m.profraw" cargo build --locked

    echo "Running tests with coverage instrumentation..."
    echo "Testing each package separately to associate binaries correctly..."
    echo ""

    # Run tests for each package separately to correctly associate test binaries
    for cur_binary in ${RUST_EXECUTABLES}; do
        echo "Testing package: ${cur_binary}"
        package_output="${PROFILING_DATA_DIR}/test_output_${cur_binary}.log"

        # Run cargo test for this specific package
        env RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="$PROFILING_DATA_DIR/coverage-%p-%m.profraw" \
            cargo test --locked --package "${cur_binary}" --manifest-path "${APP_DIR}/Cargo.toml" 2>&1 | tee "${package_output}"

        # Extract test binary paths from cargo test output for this package
        # Lines look like: "Running unittests src/lib.rs (target/debug/deps/rtemp-f17262f01034adf5)"
        # or "Running tests/test_config.rs (target/debug/deps/test_config-d3505bcdda6b431c)"
        # Output paths are always relative to the current working dir
        running_bins="$(grep -oP 'Running .* \(\K[^)]+' "${package_output}")"
        echo "${running_bins}" | sed "s|^|${APP_DIR}/|" > "$PROFILING_DATA_DIR/test_binaries_${cur_binary}.txt"

        echo ""
    done

    echo "Merging coverage data..."
    llvm-profdata merge -sparse "$PROFILING_DATA_DIR"/coverage-*.profraw -o "${MERGED_PROFDATA_FILE}"
}

# Function to print coverage summary
print_coverage_summary() {
    # Filter out external code: .cargo (dependencies), .rustup (toolchain), tests (test files), rustc (stdlib sources)
    local ignore_regex='/(\.cargo|\.rustup|tests|rustc).*/'

    echo ""
    echo "=== COVERAGE SUMMARY ==="

    # Check if profiling data exists
    if [ ! -f "${MERGED_PROFDATA_FILE}" ]; then
        echo "Error: No profiling data found. Run 'profile' first or use 'all' command."
        exit 1
    fi

    for cur_binary in ${RUST_EXECUTABLES}; do
        echo "Coverage for: ${cur_binary}"

        # Check if test binaries file exists for this package
        binaries_file="$PROFILING_DATA_DIR/test_binaries_${cur_binary}.txt"
        if [ ! -f "$binaries_file" ]; then
            echo "Warning: Test binaries list not found for ${cur_binary}, skipping..."
            echo ""
            continue
        fi

        # Read all test binaries for this package
        binary_list=$(cat "$binaries_file" | tr '\n' ' ')

        if [ -z "$binary_list" ]; then
            echo "Warning: No test binaries found for ${cur_binary}, skipping..."
            echo ""
            continue
        fi

        # Build llvm-cov command with all binaries
        # First binary is the main one, rest are passed as --object
        first_binary=$(echo "$binary_list" | awk '{print $1}')
        object_args=""
        for bin in $binary_list; do
            if [ "$bin" != "$first_binary" ]; then
                object_args="$object_args --object $bin"
            fi
        done

        cov_report_data_file="${PROFILING_DATA_DIR}/${cur_binary}.reportdata"
        cov_report_file="${PROFILING_DATA_DIR}/cov_report_${cur_binary}"
        cov_report_html_dir="${PROFILING_DATA_DIR}/html/${cur_binary}"

        mkdir -p "${cov_report_html_dir}"

        # Run llvm-cov with all test binaries
        llvm-cov report "$first_binary" $object_args \
            --instr-profile="${MERGED_PROFDATA_FILE}" \
            --ignore-filename-regex="${ignore_regex}" 2>/dev/null > "${cov_report_data_file}"

        # Generate html files
        llvm-cov show "$first_binary" $object_args \
            --instr-profile="${MERGED_PROFDATA_FILE}" \
            --ignore-filename-regex="${ignore_regex}" --format='html' --output-dir="${cov_report_html_dir}" 2>/dev/null
        echo "html coverage report can be found in: ${cov_report_html_dir}"

        # Print parsed info from the report data file
        awk "BEGIN {
                print \"File                                          Lines     Functions     Regions\"
                print \"=============================================================================\"
                total_lines_covered = 0; total_lines_total = 0
                total_funcs_covered = 0; total_funcs_total = 0
                total_regions_covered = 0; total_regions_total = 0
             }
             /.*\.rs/ {
                split(\$1,a,\"/\")
                # Columns: Filename(1), Regions(2), Missed_Regions(3), Region_Cover(4), Functions(5), Missed_Functions(6), Func_Executed(7), Lines(8), Missed_Lines(9), Line_Cover(10)
                printf \"%-40s %10s %13s %11s\n\", a[length(a)], \$10, \$7, \$4

                # Calculate totals from raw numbers
                regions_total = \$2
                regions_covered = \$2 - \$3
                funcs_total = \$5
                funcs_covered = \$5 - \$6
                lines_total = \$8
                lines_covered = \$8 - \$9

                total_lines_covered += lines_covered; total_lines_total += lines_total
                total_funcs_covered += funcs_covered; total_funcs_total += funcs_total
                total_regions_covered += regions_covered; total_regions_total += regions_total
             }
             END {
                print \"=============================================================================\"
                line_total_pct = (total_lines_total > 0) ? (total_lines_covered * 100.0 / total_lines_total) : 0
                func_total_pct = (total_funcs_total > 0) ? (total_funcs_covered * 100.0 / total_funcs_total) : 0
                region_total_pct = (total_regions_total > 0) ? (total_regions_covered * 100.0 / total_regions_total) : 0
                printf \"%-40s %9.2f%% %12.2f%% %10.2f%%\n\", \"TOTAL (filtered):\", line_total_pct, func_total_pct, region_total_pct
             }" "${cov_report_data_file}" | tee "${cov_report_file}"
        echo ""
    done


    echo ""
    echo "Coverage data stored in: $PROFILING_DATA_DIR/"
    echo "  - Raw profile files: $PROFILING_DATA_DIR/*.profraw"
    echo "  - Merged profile data: ${MERGED_PROFDATA_FILE}"
    echo "  - Test binaries per package: ${PROFILING_DATA_DIR}/test_binaries_*.txt"
    echo "  - Test output per package: ${PROFILING_DATA_DIR}/test_output_*.log"
    echo "  - llvm-cov report data: ${PROFILING_DATA_DIR}/*.reportdata"
    echo "  - llvm-cov reports: ${PROFILING_DATA_DIR}/cov_report_*"
    echo "Run with the clean argument to remove coverage files"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [clean|profile|summary|all]"
    echo "  clean    - Clean existing coverage files"
    echo "  profile  - Generate profile data from tests"
    echo "  summary  - Print coverage summary (requires existing profile data)"
    echo "  all      - Clean, generate profile data, and print summary (default)"
    echo ""
    echo "Environment variables:"
    echo "  EXCLUDE_PACKAGES - Space-separated list of packages to exclude (default: monstertruck-wasm)"
    echo "                     Example: EXCLUDE_PACKAGES='monstertruck-wasm monstertruck-gpu monstertruck-render' $0"
}

# Main function
main() {
    action="${1:-all}"

    case "$action" in
        clean)
            clean_coverage_files
            ;;
        profile)
            check_tools
            generate_profile_data
            ;;
        summary)
            check_tools
            print_coverage_summary
            ;;
        all)
            check_tools
            clean_coverage_files
            generate_profile_data
            print_coverage_summary
            ;;
        -h|--help|help)
            show_usage
            exit 0
            ;;
        *)
            echo "Error: Unknown action '$action'"
            show_usage
            exit 1
            ;;
    esac
}

RUST_EXECUTABLES="$(cargo metadata --no-deps --format-version 1 --manifest-path "${APP_DIR}/Cargo.toml" | jq -r '.packages[].name' | sort -u | xargs)"

# Filter out packages that cannot be coverage-instrumented natively
EXCLUDE_PACKAGES="${EXCLUDE_PACKAGES:-monstertruck-wasm}"
for pkg in $EXCLUDE_PACKAGES; do
    RUST_EXECUTABLES=$(echo "$RUST_EXECUTABLES" | tr ' ' '\n' | grep -v "^${pkg}$" | xargs)
done

echo "RUST_EXECUTABLES: ${RUST_EXECUTABLES}"

# Run main function with all arguments
main "$@"
