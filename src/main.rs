#![recursion_limit = "1024"]
#![allow(unused)]
use std::marker::PhantomData;
use std::any::Any;


////////// Reify //////////

trait Reify {
    fn reify() -> String;
}


////////// List //////////

struct Nil;
struct Cons<X, Xs: List>(PhantomData<X>, PhantomData<Xs>);

impl Reify for Nil {
    fn reify() -> String {
        "nil".to_string()
    }
}

impl<X, Xs> Reify for Cons<X, Xs>
where
    X:  Reify,
    Xs: List,
{
    fn reify() -> String {
        format!("{}, {}", X::reify(), Xs::reify())
    }
}

trait List: Reify {}
impl List for Nil {}
impl<X, Xs> List for Cons<X, Xs>
where
    X:  Reify,
    Xs: List
{}


////////// First //////////

trait First {
    type Output: Reify;
}

impl First for Nil {
    type Output = Nil;
}

impl<X, Xs> First for Cons<X, Xs>
where
    X:  Reify,
    Xs: List,
{
    type Output = X;
}


////////// ListConcat //////////

trait ListConcat {
    type Output: List;
}

impl<L2: List> ListConcat for (Nil, L2) {
    type Output = L2;
}

impl<X, Xs, L2> ListConcat for (Cons<X, Xs>, L2)
where
    X: Reify,
    Xs: List,
    (Xs, L2): ListConcat,
{
    type Output = Cons<X, <(Xs, L2) as ListConcat>::Output>;
}


////////// ListConcatAll //////////

trait ListConcatAll {
    type Output: List;
}

impl ListConcatAll for Nil {
    type Output = Nil;
}

impl<L, Ls> ListConcatAll for Cons<L, Ls>
where
    L:  List,
    Ls: List + ListConcatAll,
    (L, <Ls as ListConcatAll>::Output): ListConcat,
{
    type Output = <(L, <Ls as ListConcatAll>::Output) as ListConcat>::Output;
}


////////// Bool //////////

struct False;

impl Reify for False {
    fn reify() -> String {
        "false".to_string()
    }
}

struct True;

impl Reify for True {
    fn reify() -> String {
        "true".to_string()
    }
}

trait Bool: Reify {}

impl Bool for False {}
impl Bool for True {}


////////// AnyTrue //////////

trait AnyTrue {
    type Output: Bool;
}

impl AnyTrue for Nil {
    type Output = False;
}

impl<L> AnyTrue for Cons<True, L>
where
    L: List,
{
    type Output = True;
}

impl<L> AnyTrue for Cons<False, L>
where
    L: List + AnyTrue,
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

impl Reify for Z {
    fn reify() -> String {
        "0".to_string()
    }
}

struct S<N: Nat>(PhantomData<N>);

impl<N: Nat> Reify for S<N> {
    fn reify() -> String {
        (N::reify().parse::<u32>().unwrap() + 1).to_string()
    }
}

type N0 = Z;
type N1 = S<N0>;
type N2 = S<N1>;
type N3 = S<N2>;
type N4 = S<N3>;
type N5 = S<N4>;
type N6 = S<N5>;
type N7 = S<N6>;
type N8 = S<N7>;

trait Nat: Reify {}
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

impl<N> PeanoLT for (S<N>, Z)
where
    N: Nat,
{
    type Output = False;
}

impl<N> PeanoLT for (Z, S<N>)
where
    N: Nat,
{
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

impl<N> PeanoAbsDiff for (Z, S<N>)
where
    N: Nat,
{
    type Output = S<N>;
}

impl<N> PeanoAbsDiff for (S<N>, Z)
where
    N: Nat,
{
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
    type Output: List;
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
    type Output: Reify;
}

impl<A, O: Reify> Reify for dyn Apply<A, Output = O> {
    fn reify() -> String {
        <Self as Apply<A>>::Output::reify()
    }
}

struct Conj1<L>(PhantomData<L>);
impl<X, L> Apply<X> for Conj1<L>
where
    X: Reify,
    L: List,
{
    type Output = Cons<X, L>;
}


////////// Map //////////

trait Map {
    type Output: List;
}

impl<F> Map for (F, Nil) {
    type Output = Nil;
}

impl<F, X, Xs> Map for (F, Cons<X, Xs>)
where
    F:  Apply<X>,
    X:  Reify,
    Xs: List,
    (F, Xs): Map,
{
    type Output = Cons<<F as Apply<X>>::Output, <(F, Xs) as Map>::Output>;
}
 

////////// MapCat //////////

trait MapCat {
    type Output: List;
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
    type Output: List;
}

impl<X, Ys> AppendIf for (True, X, Ys)
where
    X: Reify,
    Ys: List,
{
    type Output = Cons<X, Ys>;
}

impl<X, Ys> AppendIf for (False, X, Ys)
where
    Ys: List,
{
    type Output = Ys;
}


////////// Filter //////////

trait Filter {
    type Output: List;
}

impl<F> Filter for (F, Nil) {
    type Output = Nil;
}

impl<F, X, Xs, FilterOutput> Filter for (F, Cons<X, Xs>)
where
    F: Apply<X>,
    X: Reify,
    Xs: List,
    (F, Xs): Filter<Output = FilterOutput>,
    (<F as Apply<X>>::Output, X, FilterOutput): AppendIf,
{
    type Output = <(<F as Apply<X>>::Output, X, <(F, Xs) as Filter>::Output) as AppendIf>::Output;
}


////////// Queen //////////

struct Queen<X: Reify, Y: Reify>(PhantomData<X>, PhantomData<Y>);
struct Queen1<X: Reify>(PhantomData<X>);

impl<X: Reify, Y: Reify> Reify for Queen<X, Y> {
    fn reify() -> String {
        format!("queen[x={},y={}]", X::reify(), Y::reify())
    }
}

impl<X, Y> Apply<Y> for Queen1<X>
where
    X: Reify,
    Y: Reify,
{
    type Output = Queen<X, Y>;
}


////////// QueensInRow //////////

trait QueensInRow {
    type Output: List;
}

impl<N, X> QueensInRow for (N, X)
where
    X: Reify,
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
    Ax: Reify,
    Ay: Reify,
    Bx: Reify,
    By: Reify,
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
    C: List,
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
    type Output: List;
}

impl<N, X, C> AddQueen for (N, X, C)
where
    (N, X): QueensInRow,
    (Safe1<C>, <(N, X) as QueensInRow>::Output): Filter,
    (Conj1<C>, <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output): Map,
{
    type Output = <(Conj1<C>, <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output) as Map>::Output;
}

struct AddQueen2<N, X>(PhantomData<N>, PhantomData<X>);
impl<N, X, C> Apply<C> for AddQueen2<N, X>
where
    (N, X, C): AddQueen,
{
    type Output = <(N, X, C) as AddQueen>::Output;
}


trait AddQueenToAll {
    type Output: List;
}

impl<N, X, Cs> AddQueenToAll for (N, X, Cs)
where
    (AddQueen2<N, X>, Cs): MapCat,
{
    type Output = <(AddQueen2<N, X>, Cs) as MapCat>::Output;
}


////////// AddQueensIf //////////

trait AddQueensIf {
    type Output: List;
}

impl<N, X, Cs> AddQueensIf for (False, N, X, Cs)
where
    Cs: List,
{
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
    type Output: List;
}

impl<N, X, Cs, PeanoLTOutput> AddQueens for (N, X, Cs)
where
    (X, N): PeanoLT<Output = PeanoLTOutput>,
    (PeanoLTOutput, N, X, Cs): AddQueensIf,
{
    type Output = <(<(X, N) as PeanoLT>::Output, N, X, Cs) as AddQueensIf>::Output;
}


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




////////// Testing //////////

type AreAnyTrue = <Cons<False, Cons<False, Cons<True, Cons<False, Nil>>>> as AnyTrue>::Output;
type ButNotThis = <Cons<False, Cons<False, Cons<False, Cons<False, Nil>>>> as AnyTrue>::Output;

type Insanity = <
        Cons<
            Cons<N0, Cons<N1, Cons<N2, Nil>>>,
            Cons<
                Cons<N3, Cons<N4, Cons<N5, Nil>>>,
                Cons<
                    Cons<N6, Cons<N7, Cons<N8, Nil>>>,
                    Nil,
                >
            >
        >
    as ListConcatAll>::Output;

type Range8 = <N8 as Range>::Output;


struct Sub1;
impl<N: Nat> Apply<S<N>> for Sub1 {
    type Output = N;
}

impl Apply<Z> for Sub1 {
    type Output = Z;
}

type Sub2From5 = <Sub1 as Apply<<Sub1 as Apply<N5>>::Output>>::Output;


type IsEvenFiltered = <(IsEven, Cons<N0, Cons<N1, Cons<N2, Cons<N3, Cons<N4, Cons<N5, Nil>>>>>>) as Filter>::Output;

struct IsEven;
impl Apply<N0> for IsEven {
    type Output = True;
}
impl Apply<N1> for IsEven {
    type Output = False;
}
impl<N> Apply<S<S<N>>> for IsEven
where
    N: Nat,
    IsEven: Apply<N>,
{
    type Output = <IsEven as Apply<N>>::Output;
}

fn main() {
    println!("are any true? {:?}", AreAnyTrue::reify());
    // prints "true"

    println!("but not this? {:?}", ButNotThis::reify());
    // prints "false"

    println!("nil:      {:?}", Nil::reify());
    // prints "nil"

    println!("1-elem:   {:?}", Cons::<True, Nil>::reify());
    // prints "true, nil"

    println!("list:     {:?}",
             <(
                Cons<N0, Cons<N1, Cons<N2, Nil>>>,
                Cons<N3, Cons<N4, Cons<N5, Nil>>>
              ) as ListConcat>::Output::reify());
    // prints "0, 1, 2, 3, 4, 5"

    println!("insanity: {:?}", Insanity::reify());
    // prints "0, 1, 2, 3, 4, 5, 6, 7, 8, nil"

    println!("range 8 -> 0: {:?}", Range8::reify());
    // prints "7, 6, 5, 4, 3, 2, 1, 0, nil"

    println!("5 - 2 = {:?}", Sub2From5::reify());
    // prints "3"

    println!("[1, 2, 3].map(|x| x - 1) = {:?}",
             <(
                 Sub1,
                 Cons<N1, Cons<N2, Cons<N3, Nil>>>
              ) as Map>::Output::reify());
    // prints "0, 1, 2, nil"
    
    println!("{:?}", IsEvenFiltered::reify());
    // prints "0, 2, 4, nil"
    
    println!("{:?}", std::any::type_name::<<N6 as Solution>::Output>().replace("ttti_rs::", ""));
}

