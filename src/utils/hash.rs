use nanoid::nanoid;

pub fn hash() -> String {
    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let id = nanoid!(10, &alphabet);

    id
}
