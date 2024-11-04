use example::*;
use std::collections::HashMap;

#[test]
fn test_max() {
    assert_eq!(max(&5, &10), 10);
    assert_eq!(max(&15, &7), 15);
    assert_eq!(max(&-5, &-10), -5);
    assert_eq!(max(&-5, &-5), -5);
}

#[test]
fn test_first_word() {
    assert_eq!(first_word("Hello world"), "Hello");
    assert_eq!(first_word("Rust is great"), "Rust");
    assert_eq!(first_word("SingleWord"), "SingleWord");
    assert_eq!(first_word(" Leading space"), ""); // Leading space should return an empty string
    assert_eq!(first_word(""), ""); // Empty string should also return an empty string
}

#[test]
fn p1_1() {
    let s = String::from("Hello world");
    print_str(&s);
    println!("{}", s);
}

fn print_str(a: &String) {
    println!("{}", *a);
}

#[test]
fn p1_2() {
    let mut s = String::from("Hello world");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
}

#[test]
fn p1_3() {
    let x = Box::new(5);
    let mut y = Box::new(*x);
    *y = 4;
    assert_eq!(*x, 5);
    println!("Success!")
}

#[test]
fn p2() {
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    if let Some(score) = scores.get("Sunface") {
        assert_eq!(*score, 98);
    } else {
        panic!();
    }

    match scores.get("Sunface") {
        Some(score) => println!("{score}"),
        None => println!("Not found")
    }

    if scores.contains_key("Daniel") {
        let Some(score) = scores.get("Daniel") else { panic!(); };
        assert_eq!(*score, 95);
        scores.remove("Daniel");
    }
    assert_eq!(scores.len(), 3);

    for (name, score) in scores.iter() {
        println!("{name}: {score}");
    }
}

#[test]
fn test_canvas_creation() {
    let canvas = Canvas::new(800.0, 600.0);
    assert_eq!(canvas.width, 800.0);
    assert_eq!(canvas.height, 600.0);
    assert!(canvas.shapes.is_empty());
}

#[test]
fn test_add_circle() {
    let mut canvas = Canvas::new(800.0, 600.0);
    let center = Point { x: 400.0, y: 300.0 };
    let circle = Box::new(Circle { radius: 50.0 });

    canvas.add(center, circle);
    assert_eq!(canvas.shapes.len(), 1);
}

#[test]
fn test_add_rectangle() {
    let mut canvas = Canvas::new(800.0, 600.0);
    let center = Point { x: 400.0, y: 300.0 };
    let rectangle = Box::new(Rectangle { width: 100.0, height: 50.0 });

    canvas.add(center, rectangle);
    assert_eq!(canvas.shapes.len(), 1);
}

#[test]
fn test_add_circle_out_of_bounds() {
    let mut canvas = Canvas::new(800.0, 600.0);
    let center = Point { x: 750.0, y: 590.0 };
    let circle = Box::new(Circle { radius: 50.0 });

    canvas.add(center, circle);
    assert_eq!(canvas.shapes.len(), 0); // The circle should not be added
}

#[test]
fn test_add_rectangle_out_of_bounds() {
    let mut canvas = Canvas::new(800.0, 600.0);
    let center = Point { x: 780.0, y: 575.0 };
    let rectangle = Box::new(Rectangle { width: 100.0, height: 50.0 });

    canvas.add(center, rectangle);
    assert_eq!(canvas.shapes.len(), 0); // The rectangle should not be added
}

#[test]
fn test_sum_areas() {
    let mut canvas = Canvas::new(800.0, 600.0);

    let circle = Box::new(Circle { radius: 10.0 });
    let rectangle = Box::new(Rectangle { width: 20.0, height: 30.0 });

    let circle_center = Point { x: 100.0, y: 100.0 };
    let rectangle_center = Point { x: 200.0, y: 200.0 };

    canvas.add(circle_center, circle);
    canvas.add(rectangle_center, rectangle);

    let total_area = canvas.sum_areas();
    assert_eq!(total_area, 3.14 * 10.0 * 10.0 + 20.0 * 30.0); // Circle area + Rectangle area
}

#[test]
fn test_shift_all_shapes_within_bounds() {
    let mut canvas = Canvas::new(800.0, 600.0);

    let circle = Box::new(Circle { radius: 50.0 });
    let rectangle = Box::new(Rectangle { width: 100.0, height: 50.0 });

    canvas.add(Point { x: 200.0, y: 200.0 }, circle);
    canvas.add(Point { x: 300.0, y: 300.0 }, rectangle);

    let result = canvas.shift_all(50.0, 50.0);
    assert!(result);
    assert_eq!(canvas.shapes[0].0.x, 250.0); // Circle center shifted
    assert_eq!(canvas.shapes[1].0.x, 350.0); // Rectangle center shifted
}

#[test]
fn test_shift_all_shapes_out_of_bounds() {
    let mut canvas = Canvas::new(800.0, 600.0);

    let circle = Box::new(Circle { radius: 50.0 });
    let rectangle = Box::new(Rectangle { width: 100.0, height: 50.0 });

    canvas.add(Point { x: 200.0, y: 200.0 }, circle);
    canvas.add(Point { x: 300.0, y: 300.0 }, rectangle);

    let result = canvas.shift_all(600.0, 600.0); // Shift will push out of bounds
    assert!(!result);
}

#[test]
fn test_check_in() {
    let canvas = Canvas::new(800.0, 600.0);

    let circle = Circle { radius: 50.0 };
    let rectangle = Rectangle { width: 100.0, height: 50.0 };

    let inside_circle = Point { x: 400.0, y: 300.0 };
    let outside_circle = Point { x: 500.0, y: 560.0 };

    let inside_rectangle = Point { x: 400.0, y: 300.0 };
    let outside_rectangle = Point { x: 800.0, y: 600.0 };

    assert!(canvas.check_in(&inside_circle, &circle));
    assert!(!canvas.check_in(&outside_circle, &circle));
    assert!(canvas.check_in(&inside_rectangle, &rectangle));
    assert!(!canvas.check_in(&outside_rectangle, &rectangle));
}


#[test]
fn test_person_says_hi() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    assert_eq!(person.says_hi(), "Hi, I am Alice");
}

#[test]
fn test_vec_sum() {
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(vec_sum(&vec), 15);

    let empty_vec: Vec<i32> = Vec::new();
    assert_eq!(vec_sum(&empty_vec), 0);
}

#[test]
fn test_max_min_int() {
    let vec = vec![3, 1, 4, 1, 5, 9];
    assert_eq!(max_min_int(&vec), Some((1, 9)));

    let single_element_vec = vec![42];
    assert_eq!(max_min_int(&single_element_vec), Some((42, 42)));

    let empty_vec: Vec<i32> = Vec::new();
    assert_eq!(max_min_int(&empty_vec), None);
}

#[test]
fn test_filter_a() {
    let strings = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
        String::from("date"),
        String::from("fig"),
    ];
    let result = filter_a(strings);
    assert_eq!(result, vec!["apple", "banana", "date"]);
}

#[test]
fn test_double() {
    let vec = vec![1, 2, 3, 4, 5];
    let result = double(vec);
    assert_eq!(result, vec![2, 4, 6, 8, 10]);

    let empty_vec: Vec<i32> = Vec::new();
    assert_eq!(double(empty_vec), Vec::<i32>::new());
}

#[test]
fn test_cons() {
    let list = cons(1, cons(2, cons(3, LinkedList::Nil)));
    match list {
        LinkedList::Cons(value, tail) => {
            // Ensure the first value is correct
            assert_eq!(value, 1);
            match *tail { // Dereference the tail to match on the inner LinkedList
                LinkedList::Cons(next_value, next_tail) => {
                    // Ensure the second value is correct
                    assert_eq!(next_value, 2);
                    match *next_tail {
                        LinkedList::Cons(last_value, _) => {
                            // Ensure the third value is correct
                            assert_eq!(last_value, 3);
                        }
                        _ => panic!("Expected a non-empty list for the third element"),
                    }
                }
                _ => panic!("Expected a non-empty list for the second element"),
            }
        }
        _ => panic!("Expected a non-empty list"),
    }
}

#[test]
fn test_longest() {
    let s1 = "Hello";
    let s2 = "World!";
    assert_eq!(longest(s1, s2), s2);

    let s3 = "Rust";
    let s4 = "Rocks";
    assert_eq!(longest(s3, s4), s4);

    let s5 = "Same";
    let s6 = "Size";
    assert_eq!(longest(s5, s6), s6);
}