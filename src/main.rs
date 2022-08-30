#![recursion_limit = "1024"] // necessary to generate solutions past N = 6
use std::marker::PhantomData;

////////// List //////////

struct Nil;
struct Cons<X, Xs>(PhantomData<(X, Xs)>);


////////// First //////////

trait First {
    type Output;
}

impl First for Nil {
    type Output = Nil;
}

impl<X, Xs> First for Cons<X, Xs> {
    type Output = X;
}


////////// ListConcat //////////

trait ListConcat {
    type Output;
}

impl<L2> ListConcat for (Nil, L2) {
    type Output = L2;
}

impl<X, Xs, L2> ListConcat for (Cons<X, Xs>, L2)
where
    (Xs, L2): ListConcat,
{
    type Output = Cons<X, <(Xs, L2) as ListConcat>::Output>;
}


////////// ListConcatAll //////////

trait ListConcatAll {
    type Output;
}

impl ListConcatAll for Nil {
    type Output = Nil;
}

impl<L, Ls> ListConcatAll for Cons<L, Ls>
where
    Ls: ListConcatAll,
    (L, <Ls as ListConcatAll>::Output): ListConcat,
{
    type Output = <(L, <Ls as ListConcatAll>::Output) as ListConcat>::Output;
}


////////// Bool //////////

struct False;
struct True;

trait Bool {}

impl Bool for False {}
impl Bool for True {}


////////// AnyTrue //////////

trait AnyTrue {
    type Output: Bool;
}

impl AnyTrue for Nil {
    type Output = False;
}

impl<L> AnyTrue for Cons<True, L> {
    type Output = True;
}

impl<L> AnyTrue for Cons<False, L>
where
    L: AnyTrue,
{
    type Output = <L as AnyTrue>::Output;
}


////////// Not //////////

trait Not {
    type Output: Bool;
}

impl Not for False {
    type Output = True;
}

impl Not for True {
    type Output = False;
}


////////// Or //////////

trait Or {
    type Output: Bool;
}

impl Or for (True, True) {
    type Output = True;
}

impl Or for (True, False) {
    type Output = True;
}

impl Or for (False, True) {
    type Output = True;
}

impl Or for (False, False) {
    type Output = False;
}


////////// Nats //////////

struct Z;
struct S<N: Nat>(PhantomData<N>);

type N0 = Z;
type N1 = S<N0>;
type N2 = S<N1>;
type N3 = S<N2>;
type N4 = S<N3>;
type N5 = S<N4>;
type N6 = S<N5>;

trait Nat {}
impl Nat for Z {}
impl<N: Nat> Nat for S<N> {}


////////// PeanoEqual //////////

trait PeanoEqual {
    type Output: Bool;
}

impl PeanoEqual for (Z, Z) {
    type Output = True;
}

impl<N> PeanoEqual for (Z, S<N>)
where
    N: Nat,
{
    type Output = False;
}

impl<N> PeanoEqual for (S<N>, Z)
where
    N: Nat,
{
    type Output = False;
}

impl<N1, N2> PeanoEqual for (S<N1>, S<N2>)
where
    N1: Nat,
    N2: Nat,
    (N1, N2): PeanoEqual,
{
    type Output = <(N1, N2) as PeanoEqual>::Output;
}


////////// PeanoLT //////////

trait PeanoLT {
    type Output: Bool;
}

impl PeanoLT for (Z, Z) {
    type Output = False;
}

impl<N: Nat> PeanoLT for (S<N>, Z) {
    type Output = False;
}

impl<N: Nat> PeanoLT for (Z, S<N>) {
    type Output = True;
}

impl<N1, N2> PeanoLT for (S<N1>, S<N2>)
where
    N1: Nat,
    N2: Nat,
    (N1, N2): PeanoLT,
{
    type Output = <(N1, N2) as PeanoLT>::Output;
}


////////// PeanoAbsDiff //////////

trait PeanoAbsDiff {
    type Output: Nat;
}

impl PeanoAbsDiff for (Z, Z) {
    type Output = Z;
}

impl<N: Nat> PeanoAbsDiff for (Z, S<N>) {
    type Output = S<N>;
}

impl<N: Nat> PeanoAbsDiff for (S<N>, Z) {
    type Output = S<N>;
}

impl<N1, N2> PeanoAbsDiff for (S<N1>, S<N2>)
where
    N1: Nat,
    N2: Nat,
    (N1, N2): PeanoAbsDiff,
{
    type Output = <(N1, N2) as PeanoAbsDiff>::Output;
}


////////// Range //////////

trait Range {
    type Output;
}

impl Range for Z {
    type Output = Nil;
}

impl<N> Range for S<N>
where
    N: Nat + Range,
{
    type Output = Cons<N, <N as Range>::Output>;
}


////////// Higher order functions //////////

trait Apply<A> {
    type Output;
}

struct Conj1<L>(PhantomData<L>);

impl<X, L> Apply<X> for Conj1<L> {
    type Output = Cons<X, L>;
}


////////// Map //////////

trait Map {
    type Output;
}

impl<F> Map for (F, Nil) {
    type Output = Nil;
}

impl<F, X, Xs> Map for (F, Cons<X, Xs>)
where
    F:  Apply<X>,
    (F, Xs): Map,
{
    type Output = Cons<<F as Apply<X>>::Output, <(F, Xs) as Map>::Output>;
}
 

////////// MapCat //////////

trait MapCat {
    type Output;
}

impl<F, L> MapCat for (F, L)
where
    (F, L): Map,
    <(F, L) as Map>::Output: ListConcatAll,
{
    type Output = <<(F, L) as Map>::Output as ListConcatAll>::Output;
}


////////// AppendIf //////////

trait AppendIf {
    type Output;
}

impl<X, Ys> AppendIf for (True, X, Ys) {
    type Output = Cons<X, Ys>;
}

impl<X, Ys> AppendIf for (False, X, Ys) {
    type Output = Ys;
}


////////// Filter //////////

trait Filter {
    type Output;
}

impl<F> Filter for (F, Nil) {
    type Output = Nil;
}

impl<F, X, Xs, FilterOutput> Filter for (F, Cons<X, Xs>)
where
    F: Apply<X>,
    (F, Xs): Filter<Output = FilterOutput>,
    (<F as Apply<X>>::Output, X, FilterOutput): AppendIf,
{
    type Output = <(<F as Apply<X>>::Output, X, <(F, Xs) as Filter>::Output) as AppendIf>::Output;
}


////////// Queen //////////

struct Queen<X, Y>(PhantomData<(X, Y)>);
struct Queen1<X>(PhantomData<X>);

impl<X: Nat, Y> Apply<Y> for Queen1<X> {
    type Output = Queen<X, Y>;
}


////////// QueensInRow //////////

trait QueensInRow {
    type Output;
}

impl<N, X> QueensInRow for (N, X)
where
    N: Range,
    (Queen1<X>, <N as Range>::Output): Map,
{
    type Output = <(Queen1<X>, <N as Range>::Output) as Map>::Output;
}


////////// Threatens //////////

trait Threatens {
    type Output: Bool;
}

impl<Ax, Ay, Bx, By> Threatens for (Queen<Ax, Ay>, Queen<Bx, By>)
where
    (Ax, Bx): PeanoEqual,
    (Ay, By): PeanoEqual,
    (Ax, Bx): PeanoAbsDiff,
    (Ay, By): PeanoAbsDiff,
    (<(Ax, Bx) as PeanoEqual>::Output,   <(Ay, By) as PeanoEqual  >::Output): Or,
    (<(Ax, Bx) as PeanoAbsDiff>::Output, <(Ay, By) as PeanoAbsDiff>::Output): PeanoEqual,
    (<(<(Ax, Bx) as PeanoEqual>::Output, <(Ay, By) as PeanoEqual  >::Output) as Or>::Output, <(<(Ax, Bx) as PeanoAbsDiff>::Output, <(Ay, By) as PeanoAbsDiff>::Output) as PeanoEqual>::Output): Or,
{
    type Output = <
        (
            <(
                <(Ax, Bx) as PeanoEqual>::Output,
                <(Ay, By) as PeanoEqual>::Output,
            ) as Or>::Output,
            <(
                <(Ax, Bx) as PeanoAbsDiff>::Output,
                <(Ay, By) as PeanoAbsDiff>::Output,
            ) as PeanoEqual>::Output,
        ) as Or>::Output;
}

struct Threatens1<A>(PhantomData<A>);
impl<Qa, Qb> Apply<Qb> for Threatens1<Qa>
where
    (Qa, Qb): Threatens,
{
    type Output = <(Qa, Qb) as Threatens>::Output;
}


////////// Safe //////////

trait Safe {
    type Output: Bool;
}

impl<C, Q> Safe for (C, Q)
where
    (  Threatens1<Q>, C): Map,
    <( Threatens1<Q>, C) as Map>::Output: AnyTrue,
    <<(Threatens1<Q>, C) as Map>::Output as AnyTrue>::Output: Not,
{
    type Output = <<<(Threatens1<Q>, C) as Map>::Output as AnyTrue>::Output as Not>::Output;
}

struct Safe1<C>(PhantomData<C>);
impl<C, Q> Apply<Q> for Safe1<C>
where
    (C, Q): Safe,
{
    type Output = <(C, Q) as Safe>::Output;
}


////////// AddQueen //////////

trait AddQueen {
    type Output;
}

impl<N, X, C> AddQueen for (N, X, C)
where
    (N, X): QueensInRow,
    (Safe1<C>, <(N, X) as QueensInRow>::Output): Filter,
    (Conj1<C>, <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output): Map,
{
    type Output = <(Conj1<C>, <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output) as Map>::Output;
}

struct AddQueen2<N, X>(PhantomData<(N, X)>);
impl<N, X, C> Apply<C> for AddQueen2<N, X>
where
    (N, X, C): AddQueen,
{
    type Output = <(N, X, C) as AddQueen>::Output;
}


trait AddQueenToAll {
    type Output;
}

impl<N, X, Cs> AddQueenToAll for (N, X, Cs)
where
    (AddQueen2<N, X>, Cs): MapCat,
{
    type Output = <(AddQueen2<N, X>, Cs) as MapCat>::Output;
}


////////// AddQueensIf //////////

trait AddQueensIf {
    type Output;
}

impl<N, X, Cs> AddQueensIf for (False, N, X, Cs) {
    type Output = Cs;
}

impl<N, X, Cs, AddQueenToAllOutput> AddQueensIf for (True, N, X, Cs)
where
    X: Nat,
    (N, X, Cs): AddQueenToAll<Output = AddQueenToAllOutput>,
    (N, S<X>, AddQueenToAllOutput): AddQueens,
{
    type Output = <(N, S<X>, <(N, X, Cs) as AddQueenToAll>::Output) as AddQueens>::Output;
}


trait AddQueens {
    type Output;
}

impl<N, X, Cs, PeanoLTOutput> AddQueens for (N, X, Cs)
where
    (X, N): PeanoLT<Output = PeanoLTOutput>,
    (PeanoLTOutput, N, X, Cs): AddQueensIf,
{
    type Output = <(<(X, N) as PeanoLT>::Output, N, X, Cs) as AddQueensIf>::Output;
}


////////// Solution //////////

trait Solution {
    type Output;
}

impl<N, AddQueensIfOutput> Solution for N
where
    N: Nat,
    (Z, N): PeanoLT,
    (<(Z, N) as PeanoLT>::Output, N, Z, Cons<Nil, Nil>): AddQueensIf<Output = AddQueensIfOutput>,
    AddQueensIfOutput: First,
{
    type Output = <<(N, Z, Cons<Nil, Nil>) as AddQueens>::Output as First>::Output;
}


////////// Reify //////////

fn main() {
    println!("{}", std::any::type_name::< <N6 as Solution>::Output >().replace("ttti_rs::", ""));
}
