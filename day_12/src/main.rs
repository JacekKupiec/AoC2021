use std::collections::HashMap;
use std::io;

fn main() {
    let mut buffer = String::new();
    let input = io::stdin();
    let mut graph: HashMap<&str, Vec<String>> = HashMap::new();

    while let Ok(bytes_read) = input.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Some((verticle_name1, verticle_name2)) = buffer.split_once('-') {
            let verticle_name_new1 = String::from(verticle_name1);
            let verticle_name_new2 = String::from(verticle_name2);

            if let Some(neighbour_list) = graph.get_mut(verticle_name_new1.as_str()) {
                neighbour_list.push(verticle_name_new2);
            } else {
                graph.insert(verticle_name1, vec![verticle_name_new2]);
            }

            if let Some(neighbour_list) = graph.get_mut(&verticle_name_new2.as_str()) {
                neighbour_list.push(verticle_name_new1);
            } else {
                graph.insert(verticle_name2, vec![verticle_name_new1]);
            }
        }

        buffer.clear();
    }
}
