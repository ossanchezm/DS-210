pub mod graph;
use graph::Graph;

use std::time::SystemTime;

//Function to test that everything works with a simple case
#[test]
fn test1() {
    let graph = Graph::new("test.txt", false);
    let avg_distance: f32 = average_distance(&graph);
    println!("{}", avg_distance);
    assert_eq!(avg_distance, 2.1);
}

//Function to calculate the average distance of all nodes to all other nodes in the graph
fn average_distance(graph: &Graph) -> f32{

    let mut total:u128 = 0;

    //For every node, compute the distance from that node to every other node
    for i in 0..graph.n {
        total += graph.compute_distance_bfs(i);
    }

    println!("  All distance between all nodes: {}", total);

    //Return the total divided by n*n
    (total as f64 / (graph.n as f64 * graph.n as f64)) as f32
}

fn main() {

    /*----- Directed Graph -----*/

    println!("");
    println!("DIRECTED GRAPH:");
    let before = SystemTime::now();

    //Create the graph
    let graph = Graph::new("gplus_combined.txt", true);
    let after = SystemTime::now();
    let difference = after.duration_since(before);
    let difference = difference.expect("Did the clock go back?");

    //Printouts
    println!("  Time to read file and convert it to graph: {:?}", difference);
    println!("  # of Graph Nodes (Vertices): {:?}", graph.n);


    let before = SystemTime::now();

    //Calculate the avg distance between all nodes
    let avg_distance: f32 = average_distance(&graph);

    //Printouts
    println!("  Average distance between all nodes: {:?}", avg_distance);
    let after = SystemTime::now();
    let difference = after.duration_since(before);
    let difference = difference.expect("Did the clock go back?");
    println!("  Time to compute avg distance between all nodes: {:?}", difference);

    /*----- End of Directed Graph -----*/



    /*----- Undirected Graph -----*/

    println!("");
    println!("UNDIRECTED GRAPH:");
    let before = SystemTime::now();

    //Create the graph
    let graph = Graph::new("gplus_combined.txt", false);
    let after = SystemTime::now();
    let difference = after.duration_since(before);
    let difference = difference.expect("Did the clock go back?");

    //Printouts
    println!("  Time to read file and convert it to graph: {:?}", difference);
    println!("  # of Graph Nodes (Vertices): {:?}", graph.n);


    let before = SystemTime::now();

    //Calculate the avg distance between all nodes
    let avg_distance: f32 = average_distance(&graph);

    //Printouts
    println!("  Average distance between all nodes: {:?}", avg_distance);
    let after = SystemTime::now();
    let difference = after.duration_since(before);
    let difference = difference.expect("Did the clock go back?");
    println!("  Time to compute avg distance between all nodes: {:?}", difference);

    /*----- End of Undirected Graph -----*/

}
