time cargo run --release

real	2m33.026s
user	2m29.167s
sys	0m0.955s

--- After rayon

real	2m8.248s
user	7m16.337s
sys	0m1.629s

---

Some([Play { x: 6, y: 3, dx: -1, dy: 0, count: 5 }, Play { x: 5, y: 2, dx: 0, dy: 1, count: 4 }, Play { x: 4, y: 6, dx: 1, dy: 0, count: 4 }, Play { x: 6, y: 7, dx: 0, dy: -1, count: 5 }, Play { x: 7, y: 7, dx: 0, dy: -1, count: 6 }, Play { x: 9, y: 2, dx: -1, dy: 0, count: 5 }, Play { x: 1, y: 1, dx: 0, dy: 1, count: 4 }, Play { x: 0, y: 6, dx: 1, dy: 0, count: 2 }, Play { x: 0, y: 7, dx: 1, dy: 0, count: 6 }, Play { x: 3, y: 8, dx: 0, dy: -1, count: 2 }, Play { x: 10, y: 8, dx: 0, dy: -1, count: 6 }, Play { x: 11, y: 1, dx: -1, dy: 0, count: 6 }, Play { x: 2, y: 0, dx: 0, dy: 1, count: 2 }, Play { x: 4, y: 0, dx: 0, dy: 1, count: 2 }, Play { x: 8, y: 0, dx: 0, dy: 1, count: 4 }, Play { x: 9, y: 0, dx: 0, dy: 1, count: 4 }])

