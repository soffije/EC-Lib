# EC-Lib

This code demonstrates basic operations on elliptic curve points, such as addition and doubling, using the `p256` crate in Rust. Here's a description of each function in the provided code:

1. `generate_random_point()`: This function generates a random projective point on the elliptic curve. It uses the `rand::thread_rng()` method to get a random number generator and fills a byte array with random bytes. Then, it converts the byte array to a scalar value using `p256::Scalar::from_bytes_reduced()` after converting the `&[u8; 32]` to `&[u8]`. Finally, it multiplies the generator point of the elliptic curve (`ProjectivePoint::generator()`) with the scalar to get a random projective point.

2. `add_ec_points(a: &ECPoint, b: &ECPoint) -> ECPoint`: This function takes two ECPoint structures as input (`a` and `b`) and returns their sum. It adds the corresponding projective points of `a` and `b` together and returns a new ECPoint structure with the resulting point.

3. `double_ec_point(a: &ECPoint) -> ECPoint`: This function takes an ECPoint structure (`a`) as input and returns its doubling. It doubles the projective point of `a` and returns a new ECPoint structure with the doubled point.

In the `main()` function, the code generates two random ECPoint structures (`point_a` and `point_b`) using the `generate_random_point()` function. It then calls the `add_ec_points()` function to add `point_a` and `point_b`, storing the result in the `sum` variable. Similarly, it calls the `double_ec_point()` function to double `point_a` and stores the result in the `doubled` variable. Finally, it prints the sum and doubled points using `println!()` statements.

Result:
![image](https://github.com/soffije/EC-Lib/assets/93443981/ad39e5af-5b8c-48b6-bc00-cae7d936e5ad)
