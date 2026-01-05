use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
use std::io::{self, Write};

// ============================================
// REFLECTION QUESTIONS ANSWERS (DOCUMENTATION)
// ============================================
//
// Q1: Why different sequence collections?
// ────────────────────────────────────────
// • VEC: General purpose, O(1) random access, fast push/pop at end. Use for most cases.
// • VECDEQUE: Double-ended queue, O(1) operations at both ends. Use for queues/buffers.
// • LINKEDLIST: Linked nodes, good for middle insertions. Rarely used in Rust (poor cache locality).
//
// Q2: Maps and Sets differences?
// ───────────────────────────────
// MAPS:
//   • HashMap: Unordered, O(1) average lookup. Use for general key-value lookups.
//   • BTreeMap: Ordered by keys, O(log n) lookup. Use when you need sorted order or range queries.
// SETS:
//   • HashSet: Unordered, O(1) average lookup. Use for membership testing.
//   • BTreeSet: Ordered, O(log n) lookup. Use when you need sorted iteration or ranges.
//
// Q3: BinaryHeap use cases?
// ─────────────────────────
// BinaryHeap is a MAX-HEAP (priority queue) with O(1) peek and O(log n) pop.
// Use cases:
//   • Task scheduling (high priority first)
//   • Dijkstra's shortest path algorithm
//   • Huffman coding (data compression)
//   • Event simulation (process highest priority events)
//   • Load balancing (find least busy server)
//   • K-Nearest Neighbors (ML algorithm)

fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");

    // ============================================
    // CHALLENGE 2: Basic Operations on Collections
    // ============================================
    println!("\n\n=== CHALLENGE 2: SEQUENCE COLLECTIONS ===\n");
    demo_sequences();

    // ============================================
    // CHALLENGE 3: Interactive Collection Manipulator
    // ============================================
    println!("\n\n=== CHALLENGE 3: INTERACTIVE COLLECTION MANIPULATOR ===\n");
    interactive_menu();
}

fn demo_sequences() {
    println!("--- VEC Demo ---");
    // KEY METHODS: new(), push(), len(), access by index, insert(), remove()
    let mut vec: Vec<i32> = Vec::new();
    println!("Created empty vec: {:?}", vec);
    
    vec.push(10);
    vec.push(20);
    vec.push(30);
    println!("After push(10, 20, 30): {:?}", vec);
    
    println!("Length: {}", vec.len());
    println!("Access vec[1]: {}", vec[1]);
    
    vec.insert(1, 15);
    println!("After insert(index 1, value 15): {:?}", vec);
    
    let removed = vec.remove(2);
    println!("After remove(index 2), removed value: {}", removed);
    println!("Vec now: {:?}\n", vec);

    println!("--- VECDEQUE Demo ---");
    // KEY METHODS: new(), push_front(), push_back(), pop_front(), pop_back(), back()
    let mut deque: VecDeque<i32> = VecDeque::new();
    println!("Created empty deque: {:?}", deque);
    
    deque.push_back(100);
    deque.push_back(200);
    println!("After push_back(100, 200): {:?}", deque);
    
    deque.push_front(50);
    println!("After push_front(50): {:?}", deque);
    
    if let Some(front) = deque.pop_front() {
        println!("After pop_front(), removed: {}", front);
        println!("Deque now: {:?}", deque);
    }
    
    if let Some(back) = deque.back() {
        println!("Peek at back: {}\n", back);
    }

    println!("--- LINKEDLIST Demo ---");
    // KEY METHODS: new(), push_front(), push_back(), pop_front(), pop_back(), len()
    let mut list: LinkedList<i32> = LinkedList::new();
    println!("Created empty list: {:?}", list);
    
    list.push_back(5);
    list.push_back(10);
    list.push_back(15);
    println!("After push_back(5, 10, 15): {:?}", list);
    
    list.push_front(1);
    println!("After push_front(1): {:?}", list);
    
    if let Some(front) = list.pop_front() {
        println!("After pop_front(), removed: {}", front);
        println!("List now: {:?}", list);
    }
    
    println!("Length: {}", list.len());
}

fn interactive_menu() {
    loop {
        println!("\n========== COLLECTION INTERACTIVE MENU ==========");
        println!("Choose a collection type:");
        println!("1. Vec");
        println!("2. VecDeque");
        println!("3. LinkedList");
        println!("4. HashMap");
        println!("5. BTreeMap");
        println!("6. HashSet");
        println!("7. BTreeSet");
        println!("8. BinaryHeap");
        println!("0. Exit");
        print!("\nEnter your choice (0-8): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => interact_with_vec(),
            2 => interact_with_vecdeque(),
            3 => interact_with_linkedlist(),
            4 => interact_with_hashmap(),
            5 => interact_with_btreemap(),
            6 => interact_with_hashset(),
            7 => interact_with_btreeset(),
            8 => interact_with_binaryheap(),
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select 0-8."),
        }
    }
}

fn interact_with_vec() {
    let mut vec: Vec<i32> = Vec::new();
    println!("\n--- VEC Editor ---");
    println!("Commands: add <value> | remove <index> | show | exit");
    
    loop {
        print!("vec> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        vec.push(val);
                        println!("✓ Added {}. Vec: {:?}", val, vec);
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add <value>");
                }
            }
            Some("remove") => {
                if let Some(idx_str) = parts.get(1) {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        if idx < vec.len() {
                            let removed = vec.remove(idx);
                            println!("✓ Removed {} from index {}. Vec: {:?}", removed, idx, vec);
                        } else {
                            println!("✗ Index out of bounds");
                        }
                    } else {
                        println!("✗ Invalid index");
                    }
                } else {
                    println!("✗ Usage: remove <index>");
                }
            }
            Some("show") => println!("Vec: {:?} (length: {})", vec, vec.len()),
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add, remove, show, exit"),
            None => {}
        }
    }
}

fn interact_with_vecdeque() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    println!("\n--- VECDEQUE Editor ---");
    println!("Commands: add_front <value> | add_back <value> | pop_front | pop_back | show | exit");
    
    loop {
        print!("deque> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add_front") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        deque.push_front(val);
                        println!("✓ Added {} to front. Deque: {:?}", val, deque);
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add_front <value>");
                }
            }
            Some("add_back") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        deque.push_back(val);
                        println!("✓ Added {} to back. Deque: {:?}", val, deque);
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add_back <value>");
                }
            }
            Some("pop_front") => {
                if let Some(val) = deque.pop_front() {
                    println!("✓ Removed {} from front. Deque: {:?}", val, deque);
                } else {
                    println!("✗ Deque is empty");
                }
            }
            Some("pop_back") => {
                if let Some(val) = deque.pop_back() {
                    println!("✓ Removed {} from back. Deque: {:?}", val, deque);
                } else {
                    println!("✗ Deque is empty");
                }
            }
            Some("show") => println!("Deque: {:?} (length: {})", deque, deque.len()),
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add_front, add_back, pop_front, pop_back, show, exit"),
            None => {}
        }
    }
}

fn interact_with_linkedlist() {
    let mut list: LinkedList<i32> = LinkedList::new();
    println!("\n--- LINKEDLIST Editor ---");
    println!("Commands: add_front <value> | add_back <value> | pop_front | pop_back | show | exit");
    
    loop {
        print!("list> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add_front") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        list.push_front(val);
                        println!("✓ Added {} to front. List: {:?}", val, list);
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add_front <value>");
                }
            }
            Some("add_back") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        list.push_back(val);
                        println!("✓ Added {} to back. List: {:?}", val, list);
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add_back <value>");
                }
            }
            Some("pop_front") => {
                if let Some(val) = list.pop_front() {
                    println!("✓ Removed {} from front. List: {:?}", val, list);
                } else {
                    println!("✗ List is empty");
                }
            }
            Some("pop_back") => {
                if let Some(val) = list.pop_back() {
                    println!("✓ Removed {} from back. List: {:?}", val, list);
                } else {
                    println!("✗ List is empty");
                }
            }
            Some("show") => println!("List: {:?} (length: {})", list, list.len()),
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add_front, add_back, pop_front, pop_back, show, exit"),
            None => {}
        }
    }
}

fn interact_with_hashmap() {
    let mut map: HashMap<String, i32> = HashMap::new();
    println!("\n--- HASHMAP Editor ---");
    // KEY METHODS: insert(), get(), remove(), len()
    // HASHMAP: Unordered, O(1) average, use for fast key-value lookups
    println!("Commands: add <key> <value> | get <key> | remove <key> | show | exit");
    
    loop {
        print!("map> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add") => {
                if let (Some(key), Some(val_str)) = (parts.get(1), parts.get(2)) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        map.insert(key.to_string(), val);
                        println!("✓ Set {} = {}. Map size: {}", key, val, map.len());
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add <key> <value>");
                }
            }
            Some("get") => {
                if let Some(key) = parts.get(1) {
                    if let Some(val) = map.get(*key) {
                        println!("✓ {} = {}", key, val);
                    } else {
                        println!("✗ Key not found");
                    }
                } else {
                    println!("✗ Usage: get <key>");
                }
            }
            Some("remove") => {
                if let Some(key) = parts.get(1) {
                    if let Some(val) = map.remove(*key) {
                        println!("✓ Removed {} (value was {}). Map size: {}", key, val, map.len());
                    } else {
                        println!("✗ Key not found");
                    }
                } else {
                    println!("✗ Usage: remove <key>");
                }
            }
            Some("show") => {
                if map.is_empty() {
                    println!("Map is empty");
                } else {
                    println!("Map: {:?}", map);
                }
            }
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add, get, remove, show, exit"),
            None => {}
        }
    }
}

fn interact_with_btreemap() {
    let mut map: BTreeMap<String, i32> = BTreeMap::new();
    println!("\n--- BTREEMAP Editor (Sorted) ---");
    // KEY METHODS: insert(), get(), remove(), len(), range()
    // BTREEMAP: Ordered by keys, O(log n), use when you need sorted order or range queries
    println!("Commands: add <key> <value> | get <key> | remove <key> | show | exit");
    
    loop {
        print!("btree_map> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add") => {
                if let (Some(key), Some(val_str)) = (parts.get(1), parts.get(2)) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        map.insert(key.to_string(), val);
                        println!("✓ Set {} = {}. Map size: {}", key, val, map.len());
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add <key> <value>");
                }
            }
            Some("get") => {
                if let Some(key) = parts.get(1) {
                    if let Some(val) = map.get(*key) {
                        println!("✓ {} = {}", key, val);
                    } else {
                        println!("✗ Key not found");
                    }
                } else {
                    println!("✗ Usage: get <key>");
                }
            }
            Some("remove") => {
                if let Some(key) = parts.get(1) {
                    if let Some(val) = map.remove(*key) {
                        println!("✓ Removed {} (value was {}). Map size: {}", key, val, map.len());
                    } else {
                        println!("✗ Key not found");
                    }
                } else {
                    println!("✗ Usage: remove <key>");
                }
            }
            Some("show") => {
                if map.is_empty() {
                    println!("Map is empty");
                } else {
                    println!("Map (sorted): {:?}", map);
                }
            }
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add, get, remove, show, exit"),
            None => {}
        }
    }
}

fn interact_with_hashset() {
    let mut set: HashSet<i32> = HashSet::new();
    println!("\n--- HASHSET Editor ---");
    // KEY METHODS: insert(), remove(), contains(), len()
    // HASHSET: Unordered, O(1) average, use for membership testing and deduplication
    println!("Commands: add <value> | remove <value> | contains <value> | show | exit");
    
    loop {
        print!("set> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        if set.insert(val) {
                            println!("✓ Added {}. Set size: {}", val, set.len());
                        } else {
                            println!("⚠ {} already in set", val);
                        }
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add <value>");
                }
            }
            Some("remove") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        if set.remove(&val) {
                            println!("✓ Removed {}. Set size: {}", val, set.len());
                        } else {
                            println!("✗ {} not in set", val);
                        }
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: remove <value>");
                }
            }
            Some("contains") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        if set.contains(&val) {
                            println!("✓ {} is in the set", val);
                        } else {
                            println!("✗ {} is NOT in the set", val);
                        }
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: contains <value>");
                }
            }
            Some("show") => {
                if set.is_empty() {
                    println!("Set is empty");
                } else {
                    println!("Set: {:?}", set);
                }
            }
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add, remove, contains, show, exit"),
            None => {}
        }
    }
}

fn interact_with_btreeset() {
    let mut set: BTreeSet<i32> = BTreeSet::new();
    println!("\n--- BTREESET Editor (Sorted) ---");
    // KEY METHODS: insert(), remove(), contains(), len(), range()
    // BTREESET: Ordered, O(log n), use when you need sorted iteration or range queries
    println!("Commands: add <value> | remove <value> | contains <value> | show | exit");
    
    loop {
        print!("btree_set> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        if set.insert(val) {
                            println!("✓ Added {}. Set size: {}", val, set.len());
                        } else {
                            println!("⚠ {} already in set", val);
                        }
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add <value>");
                }
            }
            Some("remove") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        if set.remove(&val) {
                            println!("✓ Removed {}. Set size: {}", val, set.len());
                        } else {
                            println!("✗ {} not in set", val);
                        }
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: remove <value>");
                }
            }
            Some("contains") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        if set.contains(&val) {
                            println!("✓ {} is in the set", val);
                        } else {
                            println!("✗ {} is NOT in the set", val);
                        }
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: contains <value>");
                }
            }
            Some("show") => {
                if set.is_empty() {
                    println!("Set is empty");
                } else {
                    println!("Set (sorted): {:?}", set);
                }
            }
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add, remove, contains, show, exit"),
            None => {}
        }
    }
}

fn interact_with_binaryheap() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    println!("\n--- BINARYHEAP Editor (Max-Heap) ---");
    // KEY METHODS: push(), pop(), peek(), len()
    // BINARYHEAP: Priority queue, O(1) peek, O(log n) pop. Max-element always at top.
    // Use cases: task scheduling, Dijkstra algorithm, Huffman coding, event simulation
    println!("Commands: add <value> | pop | peek | show | exit");
    
    loop {
        print!("heap> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("add") => {
                if let Some(val_str) = parts.get(1) {
                    if let Ok(val) = val_str.parse::<i32>() {
                        heap.push(val);
                        println!("✓ Added {}. Heap size: {}", val, heap.len());
                    } else {
                        println!("✗ Invalid number");
                    }
                } else {
                    println!("✗ Usage: add <value>");
                }
            }
            Some("pop") => {
                if let Some(val) = heap.pop() {
                    println!("✓ Removed max element: {}. Heap size: {}", val, heap.len());
                } else {
                    println!("✗ Heap is empty");
                }
            }
            Some("peek") => {
                if let Some(val) = heap.peek() {
                    println!("✓ Max element: {}", val);
                } else {
                    println!("✗ Heap is empty");
                }
            }
            Some("show") => {
                if heap.is_empty() {
                    println!("Heap is empty");
                } else {
                    println!("Heap (max at top): {:?}", heap);
                }
            }
            Some("exit") => break,
            Some(_) => println!("Unknown command. Try: add, pop, peek, show, exit"),
            None => {}
        }
    }
}
