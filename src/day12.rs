use std::collections::BTreeMap;


fn create_map(input: &str) ->(BTreeMap<&str, i32>,BTreeMap<i32, Vec<i32>>){
    let mut node_id: BTreeMap<&str, i32> = BTreeMap::from([("start",0),("end",1)]);
    let mut graph_map: BTreeMap<i32, Vec<i32>> = BTreeMap::from([(0,Vec::new()),(1,Vec::new())]);
    let mut id = 2;

    for line in input.lines(){
        let (start,end) = line.split_once('-').unwrap();
        if start != "end" && end != "start" {
            if !node_id.contains_key(start){
                node_id.insert(start, id);
                id += 1;
            }
            if !graph_map.contains_key(node_id.get(start).unwrap()){
                graph_map.insert(*node_id.get(start).unwrap(), Vec::new());
            }
            let connections = graph_map.get_mut(node_id.get(start).unwrap()).unwrap();
            if node_id.contains_key(end){
                if !connections.contains(node_id.get(end).unwrap()){
                    connections.push(*node_id.get(end).unwrap());
                }
            }
            else{
                node_id.insert(end, id);
                id += 1;
                connections.push(*node_id.get(end).unwrap());
            }

        }
    }
    for line in input.lines(){
        let (end,start) = line.split_once('-').unwrap();
        
        if end != "start" && start != "end"{
            if !graph_map.contains_key(node_id.get(start).unwrap()){
                graph_map.insert(*node_id.get(start).unwrap(), Vec::new());
            }
            let connections = graph_map.get_mut(node_id.get(start).unwrap()).unwrap();
            if node_id.contains_key(end){
                if !connections.contains(node_id.get(end).unwrap()){
                    connections.push(*node_id.get(end).unwrap());
                }
            }
            else{
                node_id.insert(end, id);
                id += 1;
                connections.push(*node_id.get(end).unwrap());
            }
        }
    }
    (node_id,graph_map)
}

fn path_finder(graph_map: &BTreeMap<i32,Vec<i32>>, assets: &mut Vec<(bool,bool)>, node: i32) -> i32{
    let mut res = 0;
    if node != 1{
        for connections in graph_map.get(&node).unwrap(){
            if assets[*connections as usize].1{
                if assets[*connections as usize].0{
                    assets[*connections as usize].1 = false;
                    res += path_finder(graph_map, assets, *connections);
                    assets[*connections as usize].1 = true;
                }
                else{
                    res += path_finder(graph_map, assets, *connections);
                }
            } 
        }
    }
    else {return 1;}
    res
}

fn path_finder_2(graph_map: &BTreeMap<i32,Vec<i32>>, assets: &mut Vec<(bool,i32)>, mut double: bool, node: i32) -> i32{
    let mut res = 0;
    if node != 1{
        for connections in graph_map.get(&node).unwrap(){
            if assets[*connections as usize].1 < 1 || double{
                if assets[*connections as usize].0{
                    assets[*connections as usize].1 += 1;
                    if assets[*connections as usize].1 == 2{ double = false;}
                    res += path_finder_2(graph_map, assets,double, *connections);
                    if assets[*connections as usize].1 == 2{ double = true;}
                    assets[*connections as usize].1 -= 1;
                }
                else{
                    res += path_finder_2(graph_map, assets,double , *connections);
                }
            } 
        }
    }
    else {return 1;}
    res
}

fn count_paths(input: &str) -> i32{
    let (node_id,graph_map) = create_map(input);
    let mut assets = node_id.iter().fold(Vec::new(), |mut acc , (a,b)| {
        acc.push((*b,a.chars().fold(true, |acc , x| acc && x.is_lowercase()),true));
        acc});
    assets.sort();
    let mut assets = assets.into_iter().map(|(_,y,z)| (y,z)).collect::<Vec<(bool,bool)>>();
    path_finder(&graph_map, &mut assets, 0)
}

fn count_paths_2(input: &str) -> i32{
    let (node_id,graph_map) = create_map(input);
    let mut assets = node_id.iter().fold(Vec::new(), |mut acc , (a,b)| {
        acc.push((*b,a.chars().fold(true, |acc , x| acc && x.is_lowercase()),0));
        acc});
    assets.sort();
    let mut assets = assets.into_iter().map(|(_,y,z)| (y,z)).collect::<Vec<(bool,i32)>>();
    path_finder_2(&graph_map, &mut assets,true , 0)
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("Paths that visit small caves at most once     -> {}",count_paths(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("Paths that visits 1 small caves at most twice -> {}",count_paths_2(&input));
}