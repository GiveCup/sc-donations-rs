# quadratic-funding-rs

## Abstract

Quadratic Funding is an algorithmic method to allocate funds in a way that promotes fair distribution and incentivizes public goods. The underlying principle is that the societal impact of public goods is proportional to the square of the sum of the square roots of individual contributions. This means that projects which are widely supported by a large number of small contributions will receive more funding than those supported by a few large ones.

By using this contract, projects can receive funds in a way that reflects both the amount and the breadth of support they have, providing a democratic approach to fund distribution.

## Calculations

Given a set of projects with individual contributions, the matched funds for a project is calculated using the following formula:

For a project `i`:
M(i) = (sum(sqrt(c(i,j)) for j=1 to n))^2

Where:
- M(i) is the matched funds for the project `i`.
- c(i,j) is the contribution of the j-th contributor to the i-th project.
- n is the total number of contributors to the project `i`.

The essence of this formula ensures that the matched funding grows based on both the amount and the breadth of the support.

## Endpoints

### init

```rust
    #[init]
    fn init(&self);
```

### addProject

```rust
    #[endpoint]
    fn add_project(&self, project_address: ManagedAddress);
```

Stores the address in the contract for it to become eligible for contributions and matched funding.

### contribute

```rust
 #[payable("*")]
    #[endpoint]
    fn contribute(&self, project_address: ManagedAddress);
```
