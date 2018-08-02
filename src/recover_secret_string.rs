use std::collections::HashMap;
use std::collections::HashSet;

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
  type Graph = HashMap<char, HashSet<char>>;

  let mut graph = triplets
    .iter()
    .fold(HashMap::new(), |mut acc: Graph, trip| {
      trip.iter().fold(None, |prev, &x| {
        let cur = acc.entry(x).or_insert(HashSet::new());
        match prev {
          Some(val) => cur.insert(val),
          None => false
        };
        Some(x)
      });

      acc
    });
    println!("{:#?}", graph);

    let mut secret = String::new();
    while graph.len() != 0 {
      let (&head, _) = graph.iter().find(|&(_, s)| s.len() == 0).unwrap();
      secret.push(head);
      graph.remove(&head);
      for (_, s) in graph.iter_mut() { s.remove(&head); }
    }

    secret
}

#[test]
fn test_recover_secret() {
  assert_eq!(
    recover_secret(vec![['a', 'b', 'c'], ['b', 'c', 'd']]),
    "abcd"
    );
  assert_eq!(
    recover_secret(vec![['b', 'c', 'd'], ['c', 'd', 'e']]),
    "bcde"
    );
}

#[test]
fn example_test() {
  assert_eq!(
    recover_secret(vec![
                   ['t', 'u', 'p'],
                   ['w', 'h', 'i'],
                   ['t', 's', 'u'],
                   ['a', 't', 's'],
                   ['h', 'a', 'p'],
                   ['t', 'i', 's'],
                   ['w', 'h', 's'],
    ]),
    "whatisup"
    );
}
