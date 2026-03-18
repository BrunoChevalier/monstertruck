use super::loops_store::ShapesOpStatus;
use monstertruck_topology::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet};

#[derive(Clone, Debug)]
pub struct FacesClassification<P, C, S> {
    shell: Shell<P, C, S>,
    status: HashMap<FaceId<S>, ShapesOpStatus>,
}

impl<P, C, S> Default for FacesClassification<P, C, S> {
    fn default() -> Self {
        Self {
            shell: Default::default(),
            status: HashMap::default(),
        }
    }
}

impl<P, C, S> FacesClassification<P, C, S> {
    pub fn push(&mut self, face: Face<P, C, S>, status: ShapesOpStatus) {
        self.status.insert(face.id(), status);
        self.shell.push(face);
    }

    pub fn and_or_unknown(&self) -> [Shell<P, C, S>; 3] {
        let [mut and, mut or, mut unknown] = <[Shell<P, C, S>; 3]>::default();
        for face in &self.shell {
            match self
                .status
                .get(&face.id())
                .expect("face id missing from status map")
            {
                ShapesOpStatus::And => and.push(face.clone()),
                ShapesOpStatus::Or => or.push(face.clone()),
                ShapesOpStatus::Unknown => unknown.push(face.clone()),
            }
        }
        [and, or, unknown]
    }

    pub fn integrate_by_component(&mut self) {
        let [and, or, unknown] = self.and_or_unknown();
        let and_boundary = and.extract_boundaries();
        let or_boundary = or.extract_boundaries();
        let and_edge_ids: FxHashSet<_> = and_boundary
            .iter()
            .flat_map(|wire| wire.edge_iter().map(|e| e.id()))
            .collect();
        let or_edge_ids: FxHashSet<_> = or_boundary
            .iter()
            .flat_map(|wire| wire.edge_iter().map(|e| e.id()))
            .collect();
        let components = unknown.connected_components();
        for comp in components {
            let boundary = comp.extract_boundaries();
            let comp_edge_ids: Vec<_> = boundary
                .iter()
                .flat_map(|wire| wire.edge_iter().map(|e| e.id()))
                .collect();
            if comp_edge_ids.is_empty() {
                // Cannot classify; leave as Unknown.
                continue;
            }
            let and_matches = comp_edge_ids
                .iter()
                .filter(|id| and_edge_ids.contains(id))
                .count();
            let or_matches = comp_edge_ids
                .iter()
                .filter(|id| or_edge_ids.contains(id))
                .count();
            if and_matches > 0 && and_matches >= or_matches {
                comp.iter().for_each(|face| {
                    *self
                        .status
                        .get_mut(&face.id())
                        .expect("face id missing from status map") = ShapesOpStatus::And;
                })
            } else if or_matches > 0 && or_matches > and_matches {
                comp.iter().for_each(|face| {
                    *self
                        .status
                        .get_mut(&face.id())
                        .expect("face id missing from status map") = ShapesOpStatus::Or;
                })
            }
            // else: tie or no matches -- leave as Unknown.
        }
    }
}

#[cfg(test)]
mod tests;
