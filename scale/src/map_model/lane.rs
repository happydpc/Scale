use crate::map_model::{IntersectionID, NavMesh, NavNodeID, Road, RoadID};
use cgmath::InnerSpace;
use cgmath::Vector2;
use serde::{Deserialize, Serialize};
use slab::Slab;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneID(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LaneType {
    Driving,
    Biking,
    Bus,
    Construction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneDirection {
    Forward,
    Backward,
}

#[derive(Serialize, Deserialize)]
pub struct Lane {
    pub id: LaneID,
    pub parent: RoadID,
    pub lane_type: LaneType,

    pub src_i: IntersectionID,
    pub dst_i: IntersectionID,

    pub src_node: Option<NavNodeID>,
    pub dst_node: Option<NavNodeID>,

    pub direction: LaneDirection,
}

impl Lane {
    pub fn set_inter_node(&mut self, id: IntersectionID, node: NavNodeID) {
        if id == self.src_i {
            self.src_node = Some(node)
        } else if id == self.dst_i {
            self.dst_node = Some(node)
        } else {
            panic!("Trying to assign node to not corresponding intersection");
        }
    }

    pub fn get_inter_node(&self, id: IntersectionID) -> NavNodeID {
        if id == self.src_i {
            self.src_node
        } else if id == self.dst_i {
            self.dst_node
        } else {
            panic!("Trying to get node to not corresponding intersection");
        }
        .expect("Lane not generated yet")
    }

    pub fn get_orientation_vec(&self, mesh: &NavMesh) -> Vector2<f32> {
        let src = mesh[self.src_node.unwrap()].pos;
        let dst = mesh[self.dst_node.unwrap()].pos;

        let vec = (dst - src).normalize();
        match self.direction {
            LaneDirection::Forward => vec,
            LaneDirection::Backward => -vec,
        }
    }

    pub fn make_forward(store: &mut Slab<Lane>, owner: &mut Road, lane_type: LaneType) {
        let entry = store.vacant_entry();
        let id = LaneID(entry.key());
        entry.insert(Lane {
            id,
            parent: owner.id,
            src_i: owner.src,
            dst_i: owner.dst,
            lane_type,
            src_node: None,
            dst_node: None,
            direction: LaneDirection::Forward,
        });
        owner.lanes_forward.push(id)
    }

    pub fn make_backward(store: &mut Slab<Lane>, owner: &mut Road, lane_type: LaneType) {
        let entry = store.vacant_entry();
        let id = LaneID(entry.key());
        entry.insert(Lane {
            id,
            parent: owner.id,
            src_i: owner.src,
            dst_i: owner.dst,
            lane_type,
            src_node: None,
            dst_node: None,
            direction: LaneDirection::Backward,
        });
        owner.lanes_backward.push(id)
    }
}