# f32 Harmonic Sum: Exact Grid Acceleration

## Experiment

Naive recurrence:

\[
S_{n+1}=\operatorname{fl}_{32}
\left(
S_n+\operatorname{fl}_{32}(1/n)
\right),
\qquad S_0=0.
\]

The naive Rust program stops at

\[
n=2^{21}=2097152,
\]

with

\[
S=15.403682708740234.
\]

The goal is to accelerate the same machine trajectory, not to approximate the mathematical harmonic number.

---

## Why accelerate only after \(S\ge 8\)?

The final trajectory lies in

\[
[8,16),
\]

so once the partial sum first enters this binade, it never leaves before numerical stopping.

For f32:

- precision \(p=24\) bits including the hidden leading 1;
- binade exponent \(e=3\);
- grid spacing

\[
h=\operatorname{ULP}=2^{e-(p-1)}=2^{-20}.
\]

Every f32 state in this binade is therefore

\[
S=jh
\]

for an integer grid index \(j\).

The naive loop reaches \(S\ge8\) after only about 1673 completed terms, so this prefix is cheap to execute literally.

---

## Grid dynamics

One exact harmonic increment has size

\[
x_n=\frac1n.
\]

Measured in grid cells,

\[
\frac{x_n}{h}
=
\frac{2^{20}}{n}.
\]

Define

\[
B=\frac2h=2^{21}.
\]

Then

\[
\frac{2x_n}{h}
=
\frac{B}{n}.
\]

Let

\[
q=\left\lfloor \frac{B}{n}\right\rfloor.
\]

Away from exact half-ULP ties, the number of grid cells jumped is

\[
k
=
\operatorname{round}\left(\frac{x_n}{h}\right)
=
\left\lfloor\frac{q+1}{2}\right\rfloor.
\]

Since \(q\) is constant over a whole interval of consecutive \(n\), so is \(k\).

If the current value of \(q\) is fixed, the largest \(r\) for which

\[
\left\lfloor\frac{B}{m}\right\rfloor=q
\]

for every \(m=n,\ldots,r\) is

\[
r=\left\lfloor\frac{B}{q}\right\rfloor.
\]

Thus the naive updates

\[
j\to j+k\to j+2k\to\cdots
\]

can be replaced by one exact block jump

\[
j\leftarrow j+(r-n+1)k.
\]

This is a compression of identical state transitions.

---

## Why the midpoint issue does not break the grouping

An addition midpoint in the grid has the form

\[
\left(m+\frac12\right)h.
\]

An exact tie requires

\[
\frac1n
=
\left(m+\frac12\right)h.
\]

Since \(h=2^{-20}\),

\[
2^{21}=(2m+1)n.
\]

The left side is a power of two. Therefore the only possible odd divisor \(2m+1\) is 1.

So the only exact midpoint tie is

\[
m=0,
\qquad
n=2^{21}.
\]

That is exactly the final half-ULP event.

The accelerator does not batch this step. It sends it back to a real f32 addition so IEEE round-to-nearest, ties-to-even decides the result.

---

## What about rounding of `1.0_f32 / n as f32`?

For all relevant \(n\),

\[
n<2^{24},
\]

so converting the integer \(n\) to f32 is exact.

The division result is correctly rounded to f32 before addition.

In the final binade \(e=3\), the nearest possible non-tie midpoint is separated from exact \(1/n\) by more than the f32 division-rounding error. Therefore the preliminary rounding of \(1/n\) cannot move the term across an addition midpoint.

So the integer quotient grouping makes the same jump/no-jump decisions as the naive recurrence.

---

## Experimental verification

Run both implementations:

1. naive f32 loop;
2. accelerated grid loop.

Then compare:

```rust
assert_eq!(naive_n, fast_n);
assert_eq!(naive_sum.to_bits(), fast_sum.to_bits());
assert_eq!(naive_term.to_bits(), fast_term.to_bits());
```

The important comparison is `to_bits()`, not an epsilon comparison.

Expected result:

\[
n=2097152,
\]

\[
S=15.403682708740234,
\]

\[
1/n=2^{-21}
=4.76837158203125\times10^{-7}.
\]

A successful assertion shows that the accelerated implementation ends in exactly the same f32 machine state as the naive loop.

This experiment validates the implementation. The grid argument above is the theoretical reason the compression is exact in the accelerated region.
