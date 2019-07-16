time cargo run --release
real	0m24.159s
user	0m24.102s
sys	0m0.392s


--- After skipping impossible PLAYABLEs + rayon

real	0m3.338s
user	0m4.903s
sys	0m0.424s
-----


Some([
Play { x: 5, y: 5, dx: -1, dy: 0, count: 3 },
Play { x: 1, y: 8, dx: 1, dy: 0, count: 2 },
Play { x: 2, y: 8, dx: 0, dy: 1, count: 1 },
Play { x: 4, y: 8, dx: 0, dy: -1, count: 2 },
Play { x: 1, y: 9, dx: 1, dy: 0, count: 2 },
Play { x: 5, y: 9, dx: 0, dy: -1, count: 2 },
Play { x: 6, y: 6, dx: -1, dy: 0, count: 3 },
Play { x: 2, y: 4, dx: 0, dy: 1, count: 2 },
Play { x: 5, y: 10, dx: -1, dy: 0, count: 3 },
Play { x: 1, y: 11, dx: 0, dy: -1, count: 3 },
Play { x: 0, y: 4, dx: 1, dy: 0, count: 1 },
Play { x: 3, y: 0, dx: 0, dy: 1, count: 4 }])