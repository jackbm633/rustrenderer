use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{Vector2, Vector3};

use crate::triangle_info::{TriangleInfo, VertexInfo};

const VERTICE_TAG: &str = "v";
const NORMAL_TAG: &str = "vn";
const UV_TAG: &str = "vt";
const TRIANGLE_TAG: &str = "f";

pub fn load_model(path: &str) -> Vec<TriangleInfo> {
    let mut triangles = Vec::<TriangleInfo>::new();

    let mut vertices = Vec::<Vector3<f32>>::new();
    let mut normal = Vec::<Vector3<f32>>::new();
    let mut uv = Vec::<Vector2<f32>>::new();

    let file = match File::open(path) {
        Ok(result) => result,
        Err(why) => panic!("{:?}", why),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let items: Vec<&str> = l.split(['/', ' ']).filter(|s| !s.is_empty()).collect();
                if let Some(item) = items.first() {
                    let item_owned = item.to_owned();
                    match item_owned {
                        VERTICE_TAG => vertices.push(Vector3::<f32>::new(
                            items[1].parse().unwrap(),
                            items[2].parse().unwrap(),
                            items[3].parse().unwrap(),
                        )),
                        NORMAL_TAG => normal.push(Vector3::<f32>::new(
                            items[1].parse().unwrap(),
                            items[2].parse().unwrap(),
                            items[3].parse().unwrap(),
                        )),
                        UV_TAG => uv.push(Vector2::<f32>::new(
                            items[1].parse().unwrap(),
                            items[2].parse().unwrap(),
                        )),
                        TRIANGLE_TAG => triangles.push(TriangleInfo {
                            vertex0: VertexInfo {
                                vertex: vertices[items[1].parse::<usize>().unwrap() - 1],
                                uv: uv[items[2].parse::<usize>().unwrap() - 1],
                                normal: normal[items[3].parse::<usize>().unwrap() - 1],
                            },
                            vertex1: VertexInfo {
                                vertex: vertices[items[4].parse::<usize>().unwrap() - 1],
                                uv: uv[items[5].parse::<usize>().unwrap() - 1],
                                normal: normal[items[6].parse::<usize>().unwrap() - 1],
                            },
                            vertex2: VertexInfo {
                                vertex: vertices[items[7].parse::<usize>().unwrap() - 1],
                                uv: uv[items[8].parse::<usize>().unwrap() - 1],
                                normal: normal[items[9].parse::<usize>().unwrap() - 1],
                            },
                        }),
                        _ => (),
                    }
                } else {
                    continue;
                }
            }
            Err(why) => panic!("{:?}", why),
        }
    }

    return triangles;
}
