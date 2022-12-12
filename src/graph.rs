//Syntactic sugar to not write types all the time
type ListOfEdges = Vec<(usize, usize)>;
type AdjacencyLists = Vec<Vec<usize>>;

//Import required modules
use std::fs::File;
use std::io::{BufRead};
use std::collections::HashMap;
use std::collections::VecDeque;


//Graph struct
pub struct Graph{

    //Nodes
    pub n : usize,

    //Adjacency list representation
    pub adjacency_list: AdjacencyLists,
}

//Graph implementation
impl Graph {

    //Function to create a new graph. If the flag is true, the graph will be directed. Else, it will be undirected
    pub fn new(path: &str, flag:bool) -> Self{
    
        //Create the edges as an empty vector
        let mut edges: ListOfEdges = Vec::new();
    
        //Read the file and iterate over the lines
        let file = File::open(path).expect("Could not open file");
        let buf_reader = std::io::BufReader::new(file).lines();

        //Create a hasmap to convert the nodes of the graph from strings to integers
        let mut pairs: HashMap<u128, usize> = HashMap::new();

        //Start the pos in 0 since the first vertex to read will be 0
        let mut pos : usize = 0;

        //For every line in the file
        for line in buf_reader {

            //Get the x and y values as u128 since they're really large
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split(' ').collect();
            let x = v[0].parse::<u128>().unwrap();
            let y = v[1].parse::<u128>().unwrap();

            //If the hashmap doesn't contains x, insert x into the hasmap
            if !pairs.contains_key(&x) {
                pairs.insert(x, pos);
                pos += 1;
            }

            //If the hashmap doesn't contains y, insert y into the hasmap
            if !pairs.contains_key(&y) {
                pairs.insert(y, pos);
                pos += 1;
            }

            //Get the integer value of the x and y value 
            let temp_x = *pairs.get(&x).unwrap();
            let temp_y =  *pairs.get(&y).unwrap();

            //Push those values into the edges
            edges.push((temp_x, temp_y));
        }

        //Create the adjagency list to store the values
        let mut graph_list : AdjacencyLists = vec![vec![]; pairs.len()];


        //For every pair of values in the edges
        for (v,w) in edges.iter() {

            //If the flag is true, just push v -> w (directed).
            if flag {
                graph_list[*v].push(*w);
            }
            //Else push v <-> w (undirected).
            else {
                graph_list[*v].push(*w);
                graph_list[*w].push(*v);
            }
        }
    
        //Return the graph
        Graph { 
            n: pairs.len() as usize, 
            adjacency_list: graph_list, 
        }
    }

    //Function to compute the distance from one node to every other node by BFS
    pub fn compute_distance_bfs(&self, start: usize) -> u128{

        //Create a vector of distances
        let mut distance: Vec<Option<u32>> = vec![None; self.n];

        //The distance from x to x is 0.
        distance[start] = Some(0);

        //Create an empty Vector Deque and push the first node
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);

        //While there are items in the queue
        while let Some(v) = queue.pop_front() { 

            //For every item in the adjacency list in that position
            for u in self.adjacency_list[v].iter() {

                //If the distance have not been calculated
                if let None = distance[*u] { 

                    //Calculate the distance
                    distance[*u] = Some(distance[v].unwrap() + 1);

                    //Push the node back
                    queue.push_back(*u);
                }
            }
        }
    
        let mut distances_acum: u128 = 0;
    
        //For every node in the graph
        for v in 0..self.n {

            //Acumulate the distance from the node passed to every other node
            if let Some(_) = distance[v] {
                distances_acum +=  distance[v].unwrap() as u128
            }
        }
        
        //Return the acumulator
        distances_acum
    }
}