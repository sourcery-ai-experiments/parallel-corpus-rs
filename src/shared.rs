// import * as record from './record'
// import * as R from 'ramda'
// import {Lens, Store} from 'reactive-lens'
// import * as Dmp from 'diff-match-patch'
// pub const dmp = new Dmp.diff_match_patch() as Dmp.diff_match_patch
pub mod union_find;
// pub type TokenDiff = [number, string][]

// interface Stringable {
//   toString: () => string
// }

// pub fn capitalize_head(s: string) {
//   return s.slice(0, 1).toUpperCase() + s.slice(1)
// }

// pub fn debug(): boolean {
//   const env = process.env.NODE_ENV
//   return env === 'production'
// }

// pub fn debugName($debugName: string) {
//   const env = process.env.NODE_ENV
//   if (env === 'production') {
//     return {}
//   } else {
//     return {$debugName}
//   }
// }

// /** Make a stream of all unicode characters

// We need this because the diff-match-patch library is hard-coded to work on characters.

// To make a polymorphic diff each unique element is assigned a unique character.
// We translate them back to the opaque type after diffing via the characters.
// This is used in `hdiff`.

//   const next = char_stream()
//   next().charCodeAt(0) = 0
//   next().charCodeAt(0) = 1
//   next().charCodeAt(0) = 2
//   next().charCodeAt(0) = 3

// */
// pub fn char_stream(): () => string {
//   let i = 0
//   return () => {
//     return String.fromCharCode(parseInt((i++).toString(), 16))
//   }
// }

// pub type ChangeInt = -1 | 0 | 1

// /**

//   raw_diff('abca'.split(''), 'bac'.split('')) // => [[-1, 'a'], [0, 'b'], [1, 'a'], [0, 'c'], [-1, 'a']]
//   raw_diff('abc'.split(''), 'cab'.split('')) // => [[1, 'c'], [0, 'a'], [0, 'b'], [-1, 'c']]
//   raw_diff('bca'.split(''), 'a1234bc'.split('')) // => [[1, 'a'], [1, '1'], [1, '2'], [1, '3'], [1, '4'], [0, 'b'], [0, 'c'], [-1, 'a']]
//   raw_diff(['anything', 'everything'], ['anything']) // => [[0, 'anything'], [-1, 'everything']]
//   const n = 10000
//   raw_diff(range(n), range(2*n)) // => range(2*n).map(i => R.pair(i < n ? 0 : 1, i))

// */
// pub fn raw_diff<A extends Stringable>(
//   xs: A[],
//   ys: A[],
//   cmp: (a: A) => string = a => a.toString()
// ): R.KeyValuePair<ChangeInt, A>[] {
//   return hdiff(xs, ys, cmp, cmp).map(c => R.pair(c.change, c.change == 1 ? c.b : c.a))
// }

// interface Deleted<A> {
//   /** -1: There was a deletion. */
//   change: -1
//   /** What was deleted. */
//   a: A
// }

// interface Constant<A, B> {
//   /** 0: There is no change in the representations of a and b. */
//   change: 0
//   /** The unchanged element in its original form. */
//   a: A
//   /** The unchanged element in its new form. */
//   b: B
// }

// interface Inserted<B> {
//   /** 1: There was an insertion. */
//   change: 1
//   /** What was inserted. */
//   b: B
// }

// pub type Change<A, B> = Deleted<A> | Constant<A, B> | Inserted<B>

// /** Heterogeneous diff

//   const abcca = 'abcca'.split('')
//   const BACC = 'BACC'.split('')
//   const lower = (s: string) => s.toLowerCase()
//   const expect = [
//     {change: -1, a: 'a'},
//     {change: 0, a: 'b', b: 'B'},
//     {change: 1, b: 'A'},
//     {change: 0, a: 'c', b: 'C'},
//     {change: 0, a: 'c', b: 'C'},
//     {change: -1, a: 'a'}
//   ] as Change<string, string>[]
//   hdiff(abcca, BACC, lower, lower) // => expect

// */
// pub fn hdiff<A extends Stringable, B extends Stringable>(
//   xs: A[],
//   ys: B[],
//   a_cmp: (a: A) => string = a => a.toString(),
//   b_cmp: (b: B) => string = b => b.toString()
// ): Change<A, B>[] {
//   const to = new Map<string, string>()
//   const a_from = new Map<string, A[]>()
//   const b_from = new Map<string, B[]>()
//   const next = char_stream()
//   fn assign<C>(c: C, c_cmp: (c: C) => string, c_from: Map<string, C[]>): string {
//     const s = c_cmp(c)
//     let u = to.get(s)
//     if (u === undefined) {
//       u = next()
//       to.set(s, u)
//     }
//     let arr = c_from.get(u)
//     if (!arr) {
//       arr = []
//       c_from.set(u, arr)
//     }
//     arr.push(c)
//     return u
//   }
//   const s1 = xs.map(a => assign(a, a_cmp, a_from)).join('')
//   const s2 = ys.map(b => assign(b, b_cmp, b_from)).join('')
//   const d = dmp.diff_main(s1, s2)
//   return flatMap(d, ([change, cs]) => {
//     return str_map(cs, (c: string) => {
//       if (change == 0) {
//         const a = (a_from.get(c) as A[]).shift() as A
//         const b = (b_from.get(c) as B[]).shift() as B
//         return {change: 0 as 0, a, b}
//       } else if (change == -1) {
//         const a = (a_from.get(c) as A[]).shift() as A
//         return {change: -1 as -1, a}
//       } else if (change == 1) {
//         const b = (b_from.get(c) as B[]).shift() as B
//         return {change: 1 as 1, b}
//       }
//       throw 'diff match patch returned change not in range [-1, 1]: ' + change
//     })
//   })
// }

use std::collections::HashSet;

/// Adds a final space if there is none
pub fn end_with_space(s: String) -> String {
    //   return s.match(/\s$/) ? s : s + ' '
    if let Some(c) = s.chars().rev().next() {
        if c.is_whitespace() {
            s
        } else {
            format!("{s} ")
        }
    } else {
        format!("{s} ")
    }
}

// pub fn token_diff(s1: string, s2: string) {
//   const d = dmp.diff_main(s1, s2)
//   dmp.diff_cleanupSemantic(d)
//   return d
// }

// pub const invert_token_diff = (ds: TokenDiff) => ds.map(([i, s]) => [-i, s] as [number, string])

// // all strings must be nonempty
// pub fn multi_token_diff(ss: string[], s2: string): TokenDiff[] {
//   if (ss.length === 0) {
//     return []
//   }
//   let lengths = ss.map(s => s.length)
//   const diff = token_diff(ss.join(''), s2)
//   let cur = [] as [number, string][]
//   const out = [cur]
//   diff.map(([i, s]) => {
//     if (i == 0 || i == -1) {
//       while (s.length > lengths[0]) {
//         const n = lengths.shift()
//         cur.push([i, s.slice(0, n)])
//         cur = []
//         out.push(cur)
//         s = s.slice(n)
//       }
//       if (s.length > 0) {
//         cur.push([i, s])
//         lengths[0] -= s.length
//       }
//     } else {
//       cur.push([i, s])
//     }
//   })
//   return out
// }

// /** Compare two arrays for shallow equality */
// pub fn shallow_array_eq<A>(xs: A[], ys: A[]): boolean {
//   return xs.length == ys.length && xs.every((x, i) => x == ys[i])
// }

// /** Check if two lists are a permutation of each other

//   array_multiset_eq(['apa', 'bepa', 'apa'], ['bepa', 'apa', 'apa', 'apa']) // => false
//   array_multiset_eq(['apa', 'bepa', 'apa'], ['bepa', 'apa', 'apa']) // => true
//   array_multiset_eq(['apa', 'bepa', 'apa'], ['bepa', 'apa']) // => false
//   array_multiset_eq(['apa', 'bepa', 'apa'], ['bepa']) // => false

// */
// pub fn array_multiset_eq<A>(xs: A[], ys: A[]): boolean {
//   const xm = new Map<A, number>()
//   const ym = new Map<A, number>()
//   let tmp
//   xs.map(x => xm.set(x, ((tmp = xm.get(x)), tmp === undefined ? 1 : tmp + 1)))
//   ys.map(y => ym.set(y, ((tmp = ym.get(y)), tmp === undefined ? 1 : tmp + 1)))
//   return map_equal(xm, ym)
// }

// /** Are these two maps equal? */
// pub fn map_equal<A, B>(a: Map<A, B>, b: Map<A, B>): boolean {
//   let ok = true
//   a.forEach((k, v) => (ok = ok && b.get(v) == k))
//   b.forEach((k, v) => (ok = ok && a.get(v) == k))
//   return ok
// }

// /** Are these two sets equal? */
// pub fn set_equal<A>(a: Set<A>, b: Set<A>): boolean {
//   let ok = true
//   a.forEach(k => (ok = ok && b.has(k)))
//   b.forEach(k => (ok = ok && a.has(k)))
//   return ok
// }

// /** Check if two lists are a permutation of each other

//   array_set_eq(['apa', 'bepa', 'apa'], ['bepa', 'apa', 'apa', 'apa']) // => true
//   array_set_eq(['apa', 'bepa', 'apa'], ['bepa', 'apa', 'apa']) // => true
//   array_set_eq(['apa', 'bepa', 'apa'], ['bepa', 'apa']) // => true
//   array_set_eq(['apa', 'bepa', 'apa'], ['bepa']) // => false

// */
// pub fn array_set_eq<A>(xs: A[], ys: A[]): boolean {
//   return set_equal(new Set(xs), new Set(ys))
// }

// /** map for strings */
// pub fn str_map<A>(s: string, f: (c: string, i: number) => A): A[] {
//   const out = [] as A[]
//   for (let i = 0; i < s.length; ++i) {
//     out.push(f(s[i], i))
//   }
//   return out
// }

// declare const require: (file: string) => any
// const stringify = require('json-stringify-pretty-compact') as (s: any) => string

// /** Show a JSON object with indentation */
// pub fn show(x: any): string {
//   return stringify(x)
//   // return JSON.stringify(x, undefined, 2)
// }

// pub fn stderr<A>(x: A): A {
//   console.error(show(x))
//   console.error()
//   return x
// }

// pub fn stdout<A>(x: A): A {
//   console.log(show(x))
//   return x
// }

// /** Numeric sort */
// pub fn numsort(xs: number[]): number[] {
//   return xs.slice().sort((u, v) => u - v)
// }

// pub type Comparator<A> = (a: A, b: A) => number

// /** Chain multiple comparators into one.

// For each pair, the returned comparator tries each sub-comparator until one returns non-0.

//   // Sort first by length, then by number of vowels, and finally (default) alphabetically
//   const xs = ['one', 'two', 'three', 'four', 'five']
//   const cmp_length = (a: string, b: string) => a.length - b.length
//   const cmp_vowels = (a: string, b: string) => a.match(/[aeiou]/g)!.length - b.match(/[aeiou]/g)!.length
//   xs.sort(chain_cmps(cmp_length, cmp_vowels)) // => ['two', 'one', 'five', 'four', 'three']
//  */
// pub fn chain_cmps<A>(...cmps: Comparator<A>[]): Comparator<A> {
//   return (a, b) => {
//     let ret = 0
//     for (const cmp of [...cmps, cmp_order]) {
//       ret = cmp(a, b)
//       if (ret != 0) break
//     }
//     return ret
//   }
// }

// /** Compare using < and >.

//   cmp_order('a', 'c') // => -1
//   cmp_order('c', 'c') // => 0
//   cmp_order('c', 'a') // => 1
//  */
// pub fn cmp_order<A>(a: A, b: A): -1 | 0 | 1 {
//   return a > b ? 1 : a < b ? -1 : 0
// }

// /** Creates a comparator which applies some fn on both elements and compares the results.

//   mkcmp((a: string) => a.length)('abc', 'abcde')       // => -1
//   mkcmp((a: string) => a.length, true)('abc', 'abcde') // => 1
//  */
// pub fn mkcmp<A>(f: (a: A) => any, negate = false): Comparator<A> {
//   return (a: A, b: A): number => (negate ? -1 : 1) * cmp_order(f(a), f(b))
// }

// /** Trims initial whitespace */
// pub fn ltrim(s: string): string {
//   const m = s.match(/^\s*(.*)$/)
//   if (m) {
//     return m[1]
//   } else {
//     return s // unreachable, the regex always matches
//   }
// }

// /** Splits a string up in the initial part and all trailing whitespace

//   whitespace_split('  XY  ') // => ['  XY', '  ']
//   whitespace_split('XY  ') // => ['XY', '  ']
//   whitespace_split('  XY') // => ['  XY', '']

// */
// pub fn whitespace_split(s: string): [string, string] {
//   const m = s.match(/^(.*?)(\s*)$/)
//   if (m && m.length == 3) {
//     return [m[1], m[2]]
//   }
//   return [s, ''] // unreachable (the regexp matches any string)
// }

// /** Splits a string up in the initial whitespace part and the rest of the string

//   initial_whitespace_split('  XY  ') // => ['  ', 'XY  ']
//   initial_whitespace_split('XY  ')   // => ['', 'XY  ']
//   initial_whitespace_split('  XY')   // => ['  ', 'XY']

// */
// pub fn initial_whitespace_split(s: string): [string, string] {
//   const m = s.match(/^(\s*)(.*)$/)
//   if (m && m.length == 3) {
//     return [m[1], m[2]]
//   }
//   return [s, ''] // unreachable (the regexp matches any string)
// }

// /** Is every element larger than the previous?

//   increases([1,2,3,4]) // => true
//   increases([1,3,4,5]) // => true
//   increases([3,1]    ) // => false
//   increases([])        // => true

// */
// pub fn increases(xs: number[]): boolean {
//   return xs.every((v, i) => i == 0 || v > xs[i - 1])
// }

// /** Is every element exactly one larger than the previous?

//   contiguous([1,2,3,4]) // => true
//   contiguous([1,3,4,5]) // => false
//   contiguous([3,1]    ) // => false
//   contiguous([])        // => true

// */
// pub fn contiguous(xs: number[]): boolean {
//   return xs.every((x, i) => i == 0 || xs[i - 1] + 1 == x)
// }

// /** Sort and partition a series into groups of contiguous items.

//   group_contiguous([1, 4, 2, 5], R.identity) // => [[1, 2], [4, 5]]
//   group_contiguous(['d', 'a', 'b'], t => t.charCodeAt(0)) // => [['a', 'b'], ['d']]

// */
// pub fn group_contiguous<A>(xs: A[], cmp: (x: A) => number) {
//   const groups: A[][] = []
//   xs.sort((a, b) => cmp(a) - cmp(b)).forEach(x => {
//     let g = groups[groups.length - 1]
//     // Contiguous with the last group, or create a new one?
//     g && cmp(g[g.length - 1]) + 1 == cmp(x) ? g.push(x) : groups.push([x])
//   })
//   return groups
// }

// /** Flatten an array of arrays */
// pub fn flatten<A>(xss: A[][]): A[] {
//   return ([] as A[]).concat(...xss)
// }

// /** Flatten an array of arrays */
// pub fn flatMap<A, B>(xs: A[], f: (a: A, index: number) => B[]): B[] {
//   return flatten(xs.map(f))
// }

// /** Get the same index of multiple arrays

//   across(1, [1, 2, 3], ['a', 'b', 'c']) // [2, 'b']
//  */
// pub fn across(i: number, ...xss: any[][]): any[] {
//   return xss.map(xs => xs[i])
// }

// /** Split an array into three pieces

//   splitAt3('0123456'.split(''), 2, 4).map(xs => xs.join('')) // => ['01', '23', '456']
//   splitAt3('0123456'.split(''), 2, 2).map(xs => xs.join('')) // => ['01', '', '23456']
//   splitAt3('0123456'.split(''), 2, 9).map(xs => xs.join('')) // => ['01', '23456', '']
//   splitAt3('0123456'.split(''), 0, 2).map(xs => xs.join('')) // => ['', '01', '23456']

// */
// pub fn splitAt3<A>(xs: A[], start: number, end: number): [A[], A[], A[]] {
//   const [ab, c] = R.splitAt(end, xs)
//   const [a, b] = R.splitAt(start, ab)
//   return [a, b, c]
// }

// /** Split an array into three pieces

//   stringSplitAt3('0123456', 2, 4) // => ['01', '23', '456']
//   stringSplitAt3('0123456', 2, 2) // => ['01', '', '23456']
//   stringSplitAt3('0123456', 2, 9) // => ['01', '23456', '']
//   stringSplitAt3('0123456', 0, 2) // => ['', '01', '23456']

// */
// pub fn stringSplitAt3(xs: string, start: number, end: number): [string, string, string] {
//   const [ab, c] = R.splitAt(end, xs)
//   const [a, b] = R.splitAt(start, ab)
//   return [a, b, c]
// }

// pub fn cat<A>(xs: (A | null)[]): A[] {
//   const out = [] as A[]
//   xs.map(x => x != null && out.push(x))
//   return out
// }

// pub fn cycle<A>(n: number, xs: A[]): A[] {
//   const out = [] as A[]
//   let i = 0
//   while (out.length < n) {
//     out.push(xs[i])
//     i++
//     if (i >= xs.length) {
//       i = 0
//     }
//   }
//   return out
// }

// /** Minimum of a non-empty array */
// pub fn minimum(xs: number[]) {
//   return xs.reduce((x, y) => Math.min(x, y), xs[0])
// }

// /** Maximum of a non-empty array */
// pub fn maximum(xs: number[]) {
//   return xs.reduce((x, y) => Math.max(x, y), xs[0])
// }

// /** Sum the numbers in an array */
// pub fn sum(xs: number[]) {
//   return xs.reduce((x, y) => x + y, 0)
// }

// /** Minimum of a non-empty array */
// pub fn minimumBy<A>(inj: (a: A) => R.Ordered, [hd, ...tl]: A[]): A {
//   return R.reduce(R.minBy(inj), hd, tl)
// }

// /** Maximum of a non-empty array */
// pub fn maximumBy<A>(inj: (a: A) => R.Ordered, [hd, ...tl]: A[]): A {
//   return R.reduce(R.maxBy(inj), hd, tl)
// }

// /** Returns a copy of the array with duplicates removed, via toString */
// pub fn uniq<A extends Stringable>(xs: A[]): A[] {
pub fn uniq<A: ToString>(xs: Vec<A>) -> Vec<A> {
    //   const seen = {} as Record<string, boolean>
    let mut seen = HashSet::new();
    xs.into_iter()
        .filter(|x| {
            let s = x.to_string();
            let duplicate = seen.contains(&s);
            seen.insert(s);
            !duplicate
        })
        .collect()
    //   return xs.filter(x => {
    //     const s = x.toString()
    //     const duplicate = s in seen
    //     seen[s] = true
    //     return !duplicate
    //   })
}

// /** Order into the result of some fn.

//   filterthese(['foo', 'bar', 'baz', 'qux'], w => w[0], ['f', 'b']) // => [['foo'], ['bar', 'baz']]
// */
// pub fn filterthese<A, B>(xs: A[], filter: (x: A) => B, outs: B[]): A[][] {
//   return outs.map(b => xs.filter(x => filter(x) === b))
// }

// /** Group into returning true and false respectively.

//   yesorno(['foo', 'bar', 'baz', 'qux'], x => x.indexOf('a') > -1) // => [['bar', 'baz'], ['foo', 'qux']]
// */
// pub fn yesorno<A>(xs: A[], filter: (x: A) => any): A[][] {
//   return filterthese(xs, x => !!filter(x), [true, false])
// }

// /** Group into same and different.

//   usandthem(['foo', 'bar', 'baz', 'qux'], x => x.indexOf('a'), -1) // => [['foo', 'qux'], ['bar', 'baz']]
// */
// pub fn usandthem<A, B>(xs: A[], filter: (x: A) => B, we: B): [A[], A[]] {
//   return yesorno(xs, x => filter(x) == we) as [A[], A[]]
// }

// /** First and last elements of an array.

//   ends(['one', 'two', 'three']) // => ['one', 'three']
//   ends(['one']) // => ['one', 'one']
//   ends([]) // => [undefined, undefined]
//  */
// pub fn ends<A>(xs: A[]): [A, A] {
//   return [xs[0], xs.slice(-1)[0]]
// }

// /** Considers the array a set and modifies the membership at some point (equality via toString)

//   const abc = 'abc'.split('')
//   set_modify(abc, 'a', true)  // => abc
//   set_modify(abc, 'a', false) // => 'bc'.split('')
//   set_modify(abc, 'd', true)  // => 'abcd'.split('')
//   set_modify(abc, 'd', false) // => abc

// */
// pub fn set_modify<A extends Stringable>(members: A[], point: A, value: boolean) {
//   if (value) {
//     return uniq([...members, point])
//   } else {
//     return members.filter(m => m.toString() != point.toString())
//   }
// }

// /** Removes adjacent elements that are equal, using === */
// pub fn drop_adjacent_equal<A>(xs: A[]): A[] {
//   return xs.filter((x, i) => i == 0 || x !== xs[i - 1])
// }

// pub fn guard<A>(p: boolean | string | undefined, x: A): A[] {
//   return p ? [x] : []
// }

// pub fn Counter<A>(xs: A[], serialize = (a: A) => JSON.stringify(a)) {
//   const count: Record<string, number> = {}
//   const insert = (x: A) => {
//     const s = serialize(x)
//     count[s] = 1 + (count[s] || 0)
//   }
//   xs.forEach(insert)
//   return (x: A) => count[serialize(x)] || 0
// }

// /**

//   const [ex, rm] = splice('abcdef'.split(''), 3, 1, ' ', '_')
//   ex.join('') // => 'abc _ef'
//   rm.join('') // => 'd'

//   const [ex, rm] = splice('abcdef'.split(''), 3, 2, ' ', '_')
//   ex.join('') // => 'abc _f'
//   rm.join('') // => 'de'

// */
// pub fn splice<A>(xs: A[], start: number, count: number, ...insert: A[]): [A[], A[]] {
//   const ys = xs.slice()
//   const zs = ys.splice(start, count, ...insert)
//   return [ys, zs]
// }

// /** True iff this fn throws an exception

//   throws(() => '123')        // => false
//   throws(() => raise('123')) // => true

// */
// pub fn throws(m: () => any): boolean {
//   try {
//     return m(), false
//   } catch (e) {
//     return true
//   }
// }

// /**

//   const u = unique_check()
//   u(1) // => true
//   u(1) // => false
//   u(1) // => false
//   u(2) // => true
//   u(3) // => true
//   u(2) // => false

// */
// pub fn unique_check<S>(): (s: S) => boolean {
//   const c = count<S>()
//   return s => c.inc(s) === 1
// }

// /**

//   const u = count()
//   u.inc(1) // => 1
//   u.inc(1) // => 2
//   u.inc(1) // => 3
//   u.inc(2) // => 1
//   u.inc(3) // => 1
//   u.inc(2) // => 2
//   u.get(1) // => 3
//   u.get(2) // => 2
//   u.get(3) // => 1

// */
// pub fn count<S>() {
//   const m = new Map<S, number>()
//   return {
//     get(s: S) {
//       return m.get(s) || 0
//     },
//     inc(s: S) {
//       m.set(s, this.get(s) + 1)
//       return this.get(s)
//     },
//   }
// }

// /** Raise an exception */
// pub fn raise<A>(s: string): A {
//   throw new Error(s)
// }

// pub fn overlaps<A>(s: Set<A>, t: Set<A>) {
//   return [...s.keys()].some(k => t.has(k))
// }

// /** Moves a slice of the items and puts back them at some destination.

//   rearrange([0, 1, 2, 3], 1, 2, 0) // => [1, 2, 0, 3]
//   rearrange([0, 1, 2, 3], 1, 2, 3) // => [0, 3, 1, 2]

//   rearrange([0, 1, 2, 3], 1, 2, 1) // => [0, 1, 2, 3]
//   rearrange([0, 1, 2, 3], 1, 2, 2) // => [0, 1, 2, 3]

// */
// pub fn rearrange<A>(xs: A[], begin: number, end: number, dest: number): A[] {
//   const [a, mid, z] = splitAt3(xs, begin, end + 1)
//   const w = end - begin
//   if (dest > begin) {
//     dest -= w
//   }
//   const [pre, post] = R.splitAt(dest, a.concat(z))
//   return pre.concat(mid, post)
// }

// /** All numbers up to and excluding the argument number

//   range(0) // => []
//   range(1) // => [0]
//   range(4) // => [0, 1, 2, 3]

// */
// pub fn range(to: number) {
//   const out = []
//   for (let i = 0; i < to; ++i) {
//     out.push(i)
//   }
//   return out
// }

// pub fn fromTo(begin: number, end: number) {
//   const out = []
//   for (let i = begin; i < end; ++i) {
//     out.push(i)
//   }
//   return out
// }

// /** Calculate the next id to use from these identifiers

//   next_id([]) // => 0
//   next_id(['t1', 't2', 't3']) // => 4
//   next_id(['u2v5k1', 'b3', 'a0']) // => 6
//   next_id(['77j66']) // => 78

// */
// pub fn next_id(xs: string[]): number {
//   let max = -1
//   xs.forEach(x => (x.match(/\d+/g) || []).forEach(i => (max = Math.max(max, parseInt(i)))))
//   return max + 1
// }

// /** Reductio ad Absurdum */
// pub fn absurd<A>(c: never): A {
//   return c
// }

// // Store stuff

// /**

//     const store = Store.init('apa bepa cepa'.split(' '))
//     const bepa = array_store_key(store, 'bepa')
//     bepa.get() // => true
//     bepa.set(false)
//     store.get() // => ['apa', 'cepa']
//     bepa.set(true)
//     store.get() // => ['apa', 'cepa', 'bepa']
//     store.set(['bepa'])
//     bepa.get() // => true
//     store.set(['bepa', 'bepa'])
//     bepa.get() // => true
//     bepa.set(true)
//     store.get() // => ['bepa']
//     bepa.set(false)
//     store.get() // => []

// This only obeys store laws if the equality of the store is relaxed to array set equality

// */
// pub fn array_store_key(store: Store<string[]>, key: string): Store<boolean> {
//   return array_store(store)
//     .via(Lens.key(key))
//     .via(Lens.iso((tu: true | undefined) => tu || false, (b: boolean) => b || undefined))
// }

// pub fn fromPairs<A extends string, B>(xs: [A, B][]): Record<A, B> {
//   return Object.assign({}, ...xs.map(([a, b]) => ({[a as string]: b})))
// }

// pub fn array_store(store: Store<string[]>): Store<Record<string, true>> {
//   return store.via(
//     Lens.iso(
//       (xs: string[]) => fromPairs(xs.map(x => [x, true] as [string, true])),
//       (r: Record<string, true>) => record.traverse(r, (_, s) => s)
//     )
//   )
// }

// /**

//     const store = Store.init('apa bepa cepa'.split(' '))
//     const str = store_join(store)
//     str.get() // => 'apa bepa cepa'
//     str.set('cepa apa bepa')
//     store.get() // => ['cepa', 'apa', 'bepa']
//     str.set('  cepa         apa     bepa  ')
//     store.get() // => ['', 'cepa', 'apa', 'bepa', '']
//     str.get() // => ' cepa apa bepa '
//     str.set('apa')
//     str.modify(x => x + ' ')
//     store.get() // => ['apa', '']
//     str.modify(x => x + 'z')
//     store.get() // => ['apa', 'z']

// This only obeys store laws if the equality of the store is relaxed about
// whitespace and strings do not mix whitespace and non-whitespace
// */
// pub fn store_join(store: Store<string[]>): Store<string> {
//   return store.via(Lens.iso((ss: string[]) => ss.join(' '), s => s.split(/\s+/g)))
// }

// pub interface RequestOptions {
//   method: 'GET' | 'POST'
//   contentType?: 'json' | 'text'
//   withCredentials?: boolean
//   headers?: Record<string, any>[]
//   data?: any
// }

// pub fn request(
//   url: string,
//   options: RequestOptions,
//   k: (response: any) => void,
//   k_err: (response: any, code: number) => void = () => {}
// ) {
//   const r = new XMLHttpRequest()
//   r.onreadystatechange = () => {
//     if (r.readyState == 4 && r.status == 200) {
//       k(r.response)
//     }
//     if (r.readyState == 4 && (r.status >= 300 || r.status == 0)) {
//       k_err(r.response, r.status)
//     }
//   }
//   r.open(options.method, url, true)
//   r.withCredentials = !!options.withCredentials
//   options.contentType == 'json' && r.setRequestHeader('Content-Type', 'application/json')
//   r.send(options.data)
// }

// /** POST request */
// pub fn POST(
//   url: string,
//   data: any,
//   k: (response: any) => void,
//   k_err: (response: any, code: number) => void = () => {}
// ): void {
//   const csrftoken = get_cookie('csrftoken')
//   const options: RequestOptions = {
//     method: 'POST',
//     contentType: 'json',
//     withCredentials: true,
//     headers: csrftoken ? [{'X-CSRFToken': csrftoken}] : [],
//     data: JSON.stringify(data),
//   }
//   request(url, options, k, k_err)
// }

// /** GET request */
// pub fn GET(
//   url: string,
//   k: (response: any) => void,
//   k_err: (response: any, code: number) => void = () => {}
// ): void {
//   const options: RequestOptions = {
//     method: 'GET',
//     contentType: 'json',
//     withCredentials: true,
//   }
//   request(url, options, k, k_err)
// }

// pub fn get_cookie(name: string): string | undefined {
//   for (const cookie of document.cookie.split(/\s*;\s*/)) {
//     if (cookie.substring(0, name.length + 1) === name + '=') {
//       return decodeURIComponent(cookie.substring(name.length + 1))
//     }
//   }
// }

// /** Debounce from underscore.js

// Returns a fn, that, as long as it continues to be invoked, will not
// be triggered. The fn will be called after it stops being called for
// N milliseconds.
// */
// pub fn debounce(wait: number, k: (...args: any[]) => void): (...args: any[]) => void {
//   let id: any | null
//   return (...args: any[]) => {
//     if (id != null) {
//       clearTimeout(id)
//     }
//     id = setTimeout(() => {
//       id = null
//       k(...args)
//     }, wait) as any
//   }
// }

// /** Iterate a fn f until a fixpoint x is reached (i.e. f(x) = x)

//   fix(1234, x => Math.round(x / 2)) // => 1
//   fix(1234, x => Math.floor(x / 2)) // => 0

// */
// pub fn fix<A>(init: A, f: (a: A) => A): A {
//   let v = init
//   let last = v
//   do {
//     last = v
//     v = f(v)
//   } while (!R.equals(v, last))
//   return v
// }

// pub fn cartesian<T, Ks extends keyof T>(r: {[K in Ks]: T[K][]}): {[K in Ks]: T[K]}[] {
//   const ks = Object.keys(r)
//   if (ks.length == 0) {
//     return [{} as any]
//   } else {
//     const k = ks[0]
//     const {[k]: vs, ...rest} = r as any
//     return flatten(cartesian(rest).map(cr => vs.map((v: any) => ({[k]: v, ...cr})))) as any
//   }
// }

// pub fn upper_triangular<A>(xs: A[]): [A, A][] {
//   const out: [A, A][] = []
//   xs.forEach((x, i) => xs.forEach((y, j) => j > i && out.push([x, y])))
//   return out
// }

// /** Very inefficient implementation of pairing up equal series

//   merge_series({x: [1,2,1,3],y: [2,1,3,1]}, sum, (a: number, b: number) => a == b) // => [{x: 3,y: 3},{x: 4,y: 4}]

//   merge_series({x: [1,1,5,3],y: [1,1,3,5]}, sum, (a: number, b: number) => a == b) // => [{x: 1,y: 1},{x: 1,y: 1},{x: 8,y: 8}]

//   merge_series({
//     x: [1,2,5,3],
//     y: [2,1,3,5],
//     z: [1,1,1,2,4,2]
//   }, sum, (a: number, b: number) => a == b) // => [{x: 3,y: 3,z: 3},{x: 8,y: 8,z: 8}]

// */
// pub fn merge_series<K extends string, S, A>(
//   r: Record<K, S[]>,
//   concat: (xs: S[]) => A,
//   cmp: (a: A, b: A) => boolean
// ): Record<K, A>[] {
//   const coords = cartesian<Record<K, number>, K>(
//     record.map(r, (ss: S[]) => ss.map((_, i) => i))
//   ).sort((a, b) => sum(Object.values(a)) - sum(Object.values(b)))
//   const prev = record.map(r, _ => 0)
//   const out: Record<K, A>[] = []
//   coords.forEach(coord => {
//     if (record.traverse(coord, (i: number, k: K) => i >= prev[k]).every((b: boolean) => b)) {
//       const a = record.map(r, (s, k) => concat(fromTo(prev[k], coord[k] + 1).map(i => s[i])))
//       if (upper_triangular<A>(Object.values(a)).every(([x, y]) => cmp(x, y))) {
//         record.forEach(coord, (i: number, k: K) => (prev[k] = i + 1))
//         out.push(a)
//       }
//     }
//   })
//   return out
// }

// pub fn zipWithPrevious<A, B>(
//   xs: A[],
//   k: (x: A, prev: A | undefined, i: number) => B
// ): B[] {
//   return xs.map((x, i) => k(x, xs[i - 1], i))
// }

// pub interface KV<K, V> {
//   has(k: K): boolean
//   get(k: K): V | undefined
//   set(k: K, v: V): void
//   forEach(f: (v: V, k: K) => void): void
//   batch(kvs: {key: K; value: V}[]): void
//   obj: Record<string, V>
// }

// pub fn KV<K, V>(s: (k: K) => string = JSON.stringify): KV<K, V> {
//   const obj = {} as Record<string, V>
//   const krev = {} as Record<string, K>
//   const api: KV<K, V> = {
//     has(k: K) {
//       return s(k) in obj
//     },
//     get(k: K): V | undefined {
//       return obj[s(k)]
//     },
//     set(k: K, v: V) {
//       obj[s(k)] = v
//       krev[s(k)] = k
//     },
//     forEach(f) {
//       Object.keys(obj).map(sk => f(obj[sk], krev[sk]))
//     },
//     batch(kvs) {
//       kvs.map(m => api.set(m.key, m.value))
//     },
//     obj,
//   }
//   return api
// }

// pub type LazySnocList<A> = (() => Snoc<A>) | null
// pub interface Snoc<A> {
//   0: LazySnocList<A>
//   1: A
// }
// pub fn snoc<A>(xs: LazySnocList<A>, x: A): LazySnocList<A> {
//   return () => [xs, x]
// }

// pub fn snocs<A>(xs: LazySnocList<A>, ys: A[]): LazySnocList<A> {
//   return ys.reduce((xs, y) => snoc(xs, y), xs)
// }

// pub fn snocsToArray<A>(xs: LazySnocList<A>): A[] {
//   const out: A[] = []
//   while (xs !== null) {
//     const cell = xs()
//     out.push(cell[1])
//     xs = cell[0]
//   }
//   return out.reverse()
// }

// pub fn expr<R>(k: () => R): R {
//   return k()
// }

// pub fn chain<A, B>(a: A, f: (a: A) => B): B {
//   return f(a)
// }

// /** Push an object to a record of lists.

//   const a = {}
//   push(a, 'a', 'A')
//   a // => {a: ['A']}
//   push(a, 'a', 'B')
//   a // => {a: ['A', 'B']}
//  */
// pub fn push<K extends string, V>(obj: Record<K, V[]>, k: string, ...vs: V[]) {
//   const _obj = (obj as any) as Record<string, V[]>
//   ;(_obj[k] || (_obj[k] = [])).push(...vs)
// }

// pub fn setIfChanged<A>(store: Store<A>, value: A) {
//   const now = store.get()
//   if (!R.equals(now, value)) {
//     store.set(value)
//   }
// }

// /**

//   edit_range('0123456789', '0189') // => {from: 2, to: 8, insert: ''}
//   edit_range('0123456789', '01') // => {from: 2, to: 10, insert: ''}
//   edit_range('0123456789', '89') // => {from: 0, to: 8, insert: ''}
//   edit_range('0123456789', '') // => {from: 0, to: 10, insert: ''}

//   edit_range('0123456789', '01xyz89') // => {from: 2, to: 8, insert: 'xyz'}
//   edit_range('0123456789', '01xyz') // => {from: 2, to: 10, insert: 'xyz'}
//   edit_range('0123456789', 'xyz89') // => {from: 0, to: 8, insert: 'xyz'}
//   edit_range('0123456789', 'xyz') // => {from: 0, to: 10, insert: 'xyz'}

//   edit_range('', '01') // => {from: 0, to: 0, insert: '01'}

// */
// pub fn edit_range(s0: string, s: string): {from: number; to: number; insert: string} {
//   const patches = token_diff(s0, s)
//   const pre = R.takeWhile<[number, string]>(i => i[0] == 0, patches)
//   const post = R.takeLastWhile<[number, string]>(i => i[0] == 0, R.drop(pre.length, patches))
//   const from = pre.map(i => i[1]).join('').length
//   const postlen = post.map(i => i[1]).join('').length
//   const to = s0.length - postlen
//   const insert = s.slice(from, s.length - (s0.length - to))
//   return {from, to, insert}
// }

// pub fn within(lo: number, x: number, hi: number) {
//   return lo <= x && x < hi
// }

// /**

//   findLastIndex(range(10), x => x % 2 == 0) // => 8
//   findLastIndex(range(10), x => x % 2 == 1) // => 9

//   // returns xs.length if not found:
//   findLastIndex(range(10), x => x > 100) // => 10

// */
// pub fn findLastIndex<A>(xs: A[], f: (v: A, i: number) => boolean): number {
//   const op = (rindex: number) => xs.length - rindex - 1
//   return op(xs.findIndex((_, rindex) => f(xs[op(rindex)], op(rindex))))
// }

// pub fn timeit<A>(label: string, doit: () => A): A {
//   const yes = false
//   yes && console.time(label)
//   const a = doit()
//   yes && console.timeEnd(label)
//   return a
// }

// pub fn getUnsafe<K, V>(m: Map<K, V>, k: K): V {
//   return m.get(k) || raise(`Key missing: ${k}`)
// }

// pub fn any(...bs: boolean[]): boolean {
//   for (const b of bs) {
//     if (b) {
//       return true
//     }
//   }
//   return false
// }

// /** Scroll a parent element vertically to include a child element.

// Parent must have `position: relative`. */
// pub fn scrollIntoView(parent: HTMLElement, child: HTMLElement) {
//   const scrollY = (top: number) => parent.scroll({top, behavior: 'smooth'})
//   // If child bottom is below parent bottom, align bottoms.
//   if (child.offsetTop + child.offsetHeight > parent.scrollTop + parent.offsetHeight)
//     scrollY(child.offsetTop + child.offsetHeight - parent.offsetHeight)
//   // If child top is above parent top, align tops.
//   if (child.offsetTop < parent.scrollTop) scrollY(child.offsetTop)
// }
