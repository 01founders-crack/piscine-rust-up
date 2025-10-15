# piscine-rust-up
This is originally a dump of the /app folder of the [test-rust docker image from 01-edu](https://github.com/01-edu/rust-tests/pkgs/container/test-rust).

It's the pedagogical and technical backbone of this 01-style piscine, which, unlike 42, is moulinette-only.  
Its general idea is simple :
- Start the first quest from the curriculum below
- Do the first exercise
- Test it with the included automated test
- If good, move on to the next one
- ...
- Congratulations, you are now a rust developer.

Though in the real program:
- each Day... uuh I mean, Quest becomes available the day after the other, 
- there is a 3 minutes cooldown between each moulinette test
- you're supposed to do the 2 rushes... uhh I mean, raids in groups of 3 and be audited manually by staff
- the practical project `chaikin` is (the only project) to be reviewed by peers.

# Instructions

Make sure the Rust toolchain is installed on your computer.  
It is best to install it the official way, by going [here](https://www.rust-lang.org/learn/get-started), and not through your package manager.

Make sure your whole piscine repo is the `solutions` folder alongside `tests`, at the root of this repository.  
This should look like this:  
`piscine-rust/ <- you are here!`  
`------------/tests`  
`------------/solutions/fibonacci2/<contents of the cargo module>`

You can launch the automated testing of a completed exercise at the root of this repository with the command `./test.sh <exercise_name>`.  
You can also directly run `cargo test --manifest-path tests/<exercise_name>_test/Cargo.toml`.

Feel free to fork this repository, remove `solutions/*` from `.gitignore` and get to work directly in it.  
Have fun!

# Notes

The original test mechanism was the following:
- The student repo was cloned under `/jail/student` by the 01 runner program
- The container is run, `$EXERCISE` and such are filled and `/jail` is set as current directory
- `/app/entrypoint.sh` (reworked into `test.sh`) is called upon running the container
- `/app/tests` is copied in `/jail` so that it is alongside the student repo
- `/jail/student` is renamed into `/jail/solutions` because runner clones the repo as "`student`" yet all rust tests are written to look for a "`solutions`" package :facepalm:  
	NB: I have yet no idea if the `/app/tests_utility` is necessary since it is never copied in `/jail/student` during testing


# Curriculum

## Week One

### Quest 01-rust

Video: [https://www.youtube.com/watch?v=gjGzMMUdKDM](https://www.youtube.com/watch?v=gjGzMMUdKDM)

***

Exercises:

- **scalar** | [https://github.com/01-edu/public/tree/master/subjects/scalar](https://github.com/01-edu/public/tree/master/subjects/scalar)
- **temperature_conv** | [https://github.com/01-edu/public/tree/master/subjects/temperature_conv](https://github.com/01-edu/public/tree/master/subjects/temperature_conv)
- **speed_transformation** | [https://github.com/01-edu/public/tree/master/subjects/speed_transformation](https://github.com/01-edu/public/tree/master/subjects/speed_transformation)
- **looping** | [https://github.com/01-edu/public/tree/master/subjects/looping](https://github.com/01-edu/public/tree/master/subjects/looping)
- **groceries** | [https://github.com/01-edu/public/tree/master/subjects/groceries](https://github.com/01-edu/public/tree/master/subjects/groceries)
- **reverse_string** | [https://github.com/01-edu/public/tree/master/subjects/reverse_string](https://github.com/01-edu/public/tree/master/subjects/reverse_string)
- **find_factorial** | [https://github.com/01-edu/public/tree/master/subjects/find_factorial](https://github.com/01-edu/public/tree/master/subjects/find_factorial)
- **fibonacci2** | [https://github.com/01-edu/public/tree/master/subjects/fibonacci2](https://github.com/01-edu/public/tree/master/subjects/fibonacci2)
- **matrix_transposition** | [https://github.com/01-edu/public/tree/master/subjects/matrix_transposition](https://github.com/01-edu/public/tree/master/subjects/matrix_transposition)
- **division_and_remainder** | [https://github.com/01-edu/public/tree/master/subjects/division_and_remainder](https://github.com/01-edu/public/tree/master/subjects/division_and_remainder)
- **tuples_refs** | [https://github.com/01-edu/public/tree/master/subjects/tuples_refs](https://github.com/01-edu/public/tree/master/subjects/tuples_refs)

### Quest 02-rust

Video: https://www.youtube.com/watch?v=O0o19HANB_w

Exercises:

  - **borrow** | [https://github.com/01-edu/public/tree/master/subjects/borrow](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/borrow)
  - **doubtful** | [https://github.com/01-edu/public/tree/master/subjects/doubtful](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/doubtful)
  - **to\_url** | [https://github.com/01-edu/public/tree/master/subjects/to\_url](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/to_url)
  - **string\_literals** | [https://github.com/01-edu/public/tree/master/subjects/string\_literals](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/string_literals)
  - **name\_initials** | [https://github.com/01-edu/public/tree/master/subjects/name\_initials](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/name_initials)
  - **ownership** | [https://github.com/01-edu/public/tree/master/subjects/ownership](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/ownership)
  - **copy** | [https://github.com/01-edu/public/tree/master/subjects/copy](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/copy)
  - **borrow\_me\_the\_reference** | [https://github.com/01-edu/public/tree/master/subjects/borrow\_me\_the\_reference](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/borrow_me_the_reference)
  - **tic\_tac\_toe** | [https://github.com/01-edu/public/tree/master/subjects/tic\_tac\_toe](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/tic_tac_toe)
  - **arrange\_it** | [https://github.com/01-edu/public/tree/master/subjects/arrange\_it](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/arrange_it)


### Quest 03-rust

Video: https://www.youtube.com/watch?v=URAJIouRd0I


Exercises:

  - **changes** | [https://github.com/01-edu/public/tree/master/subjects/changes](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/changes)
  - **circle** | [https://github.com/01-edu/public/tree/master/subjects/circle](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/circle)
  - **card\_deck** | [https://github.com/01-edu/public/tree/master/subjects/card\_deck](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/card_deck)
  - **arrays** | [https://github.com/01-edu/public/tree/master/subjects/arrays](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/arrays)
  - **strings** | [https://github.com/01-edu/public/tree/master/subjects/strings](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/strings)
  - **capitalizing** | [https://github.com/01-edu/public/tree/master/subjects/capitalizing](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/capitalizing)
  - **edit\_distance** | [https://github.com/01-edu/public/tree/master/subjects/edit\_distance](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/edit_distance)
  - **simple\_hash** | [https://github.com/01-edu/public/tree/master/subjects/simple\_hash](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/simple_hash)
  - **bigger** | [https://github.com/01-edu/public/tree/master/subjects/bigger](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/bigger)
  - **string\_permutation** | [https://github.com/01-edu/public/tree/master/subjects/string\_permutation](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/string_permutation)
  - **hashing** | [https://github.com/01-edu/public/tree/master/subjects/hashing](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/hashing)
  - **collect** | [https://github.com/01-edu/public/tree/master/subjects/collect](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/collect)


### Raid 01-rust

Exercice:

- drawing | https://github.com/01-edu/public/tree/master/subjects/drawing

---

## Week Two

### Quest 04-rust

Video: https://www.youtube.com/watch?v=3D0mOw0egHc

Exercises:

  - **unwrap\_or\_expect** | [https://github.com/01-edu/public/tree/master/subjects/unwrap\_or\_expect](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/unwrap_or_expect)
  - **panic** | [https://github.com/01-edu/public/tree/master/subjects/panic](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/panic)
  - **handling** | [https://github.com/01-edu/public/tree/master/subjects/handling](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/handling)
  - **profanity\_filter** | [https://github.com/01-edu/public/tree/master/subjects/profanity\_filter](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/profanity_filter)
  - **question\_mark** | [https://github.com/01-edu/public/tree/master/subjects/question\_mark](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/question_mark)
  - **banner** | [https://github.com/01-edu/public/tree/master/subjects/banner](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/banner)
  - **cipher** | [https://github.com/01-edu/public/tree/master/subjects/cipher](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/cipher)
  - **error\_types** | [https://github.com/01-edu/public/tree/master/subjects/error\_types](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/error_types)
  - **boxing\_todo** | [https://github.com/01-edu/public/tree/master/subjects/boxing\_todo](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/boxing_todo)



### Quest 05-rust

Video: https://www.youtube.com/watch?v=XyUliQD7v-0

Exercises:

  - **middle\_day** | [https://github.com/01-edu/public/tree/master/subjects/middle\_day](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/middle_day)
  - **does\_it\_fit** | [https://github.com/01-edu/public/tree/master/subjects/does\_it\_fit](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/does_it_fit)
  - **macro\_calculator** | [https://github.com/01-edu/public/tree/master/subjects/macro\_calculator](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/macro_calculator)
  - **shopping\_mall** | [https://github.com/01-edu/public/tree/master/subjects/shopping\_mall](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/shopping_mall)
  - **expected\_variable** | [https://github.com/01-edu/public/tree/master/subjects/expected\_variable](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/expected_variable)
  - **mobs** | [https://github.com/01-edu/public/tree/master/subjects/mobs](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/mobs)

### Quest 06-rust

Video: [https://www.youtube.com/watch?v=LMOcoPamcxM](https://www.youtube.com/watch?v=LMOcoPamcxM)

-----

Exercises:

  - **stars** | [https://github.com/01-edu/public/tree/master/subjects/stars](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/stars)
  - **scores** | [https://github.com/01-edu/public/tree/master/subjects/scores](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/scores)
  - **searching** | [https://github.com/01-edu/public/tree/master/subjects/searching](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/searching)
  - **ordinal** | [https://github.com/01-edu/public/tree/master/subjects/ordinal](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/ordinal)
  - **pangram** | [https://github.com/01-edu/public/tree/master/subjects/pangram](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/pangram)
  - **talking** | [https://github.com/01-edu/public/tree/master/subjects/talking](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/talking)
  - **logic\_number** | [https://github.com/01-edu/public/tree/master/subjects/logic\_number](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/logic_number)
  - **rot** | [https://github.com/01-edu/public/tree/master/subjects/rot](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/rot)
  - **rgb\_match** | [https://github.com/01-edu/public/tree/master/subjects/rgb\_match](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/rgb_match)
  - **pig\_latin** | [https://github.com/01-edu/public/tree/master/subjects/pig\_latin](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/pig_latin)
  - **scytale\_cipher** | [https://github.com/01-edu/public/tree/master/subjects/scytale\_cipher](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/scytale_cipher)
  - **diamond\_creation** | [https://github.com/01-edu/public/tree/master/subjects/diamond\_creation](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/diamond_creation)
  - **spelling** | [https://github.com/01-edu/public/tree/master/subjects/spelling](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/spelling)

---

## Week Three

### Quest 07-rust

Video: [https://www.youtube.com/watch?v=jzgKZTtVNwQ](https://www.youtube.com/watch?v=jzgKZTtVNwQ)

-----

Exercises:

  - **box\_it** | [https://github.com/01-edu/public/tree/master/subjects/box\_it](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/box_it)
  - **borrow\_box** | [https://github.com/01-edu/public/tree/master/subjects/borrow\_box](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/borrow_box)
  - **box\_recursion** | [https://github.com/01-edu/public/tree/master/subjects/box\_recursion](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/box_recursion)
  - **how\_many\_references** | [https://github.com/01-edu/public/tree/master/subjects/how\_many\_references](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/how_many_references)
  - **ref\_cell** | [https://github.com/01-edu/public/tree/master/subjects/ref\_cell](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/ref_cell)
  - **drop\_the\_thread** | [https://github.com/01-edu/public/tree/master/subjects/drop\_the\_thread](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/drop_the_thread)


### Quest 08-rust

Video: [https://www.youtube.com/watch?v=xCEDVb4p7Js](https://www.youtube.com/watch?v=xCEDVb4p7Js)

-----

Exercises:

  - **generics** | [https://github.com/01-edu/public/tree/master/subjects/generics](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/generics)
  - **generics\_list** | [https://github.com/01-edu/public/tree/master/subjects/generics\_list](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/generics_list)
  - **easy\_traits** | [https://github.com/01-edu/public/tree/master/subjects/easy\_traits](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/easy_traits)
  - **traits** | [https://github.com/01-edu/public/tree/master/subjects/traits](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/traits)
  - **blood\_types** | [https://github.com/01-edu/public/tree/master/subjects/blood\_types](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/blood_types)
  - **vector\_operations** | [https://github.com/01-edu/public/tree/master/subjects/vector\_operations](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/vector_operations)
  - **lalgebra\_scalar** | [https://github.com/01-edu/public/tree/master/subjects/lalgebra\_scalar](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/lalgebra_scalar)
  - **lalgebra\_vector** | [https://github.com/01-edu/public/tree/master/subjects/lalgebra\_vector](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/lalgebra_vector)
  - **commits\_stats** | [https://github.com/01-edu/public/tree/master/subjects/commits\_stats](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/commits_stats)
  - **roman\_numbers** | [https://github.com/01-edu/public/tree/master/subjects/roman\_numbers](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/roman_numbers)


### Project 01-rust

Exercice:
- chaikin | https://github.com/01-edu/public/tree/master/subjects/chaikin

---

## Week Four

### Quest 09-rust

Video: [https://www.youtube.com/watch?v=xCEDVb4p7Js](https://www.youtube.com/watch?v=xCEDVb4p7Js)

Exercises:

  - **matrix** | [https://github.com/01-edu/public/tree/master/subjects/matrix](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/matrix)
  - **matrix\_ops** | [https://github.com/01-edu/public/tree/master/subjects/matrix\_ops](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/matrix_ops)
  - **matrix\_mult** | [https://github.com/01-edu/public/tree/master/subjects/matrix\_mult](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/matrix_mult)
  - **lifetimes** | [https://github.com/01-edu/public/tree/master/subjects/lifetimes](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/lifetimes)
  - **delete\_prefix** | [https://github.com/01-edu/public/tree/master/subjects/delete\_prefix](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/delete_prefix)
  - **border\_cross** | [https://github.com/01-edu/public/tree/master/subjects/border\_cross](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/border_cross)
  - **events** | [https://github.com/01-edu/public/tree/master/subjects/events](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/events)

### Quest 10-rust

Video: [https://www.youtube.com/watch?v=sdEEmmlI6K0](https://www.youtube.com/watch?v=sdEEmmlI6K0)

Exercises:

  - **closures** | [https://github.com/01-edu/public/tree/master/subjects/closures](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/closures)
  - **sales** | [https://github.com/01-edu/public/tree/master/subjects/sales](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/sales)
  - **adding** | [https://github.com/01-edu/public/tree/master/subjects/adding](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/adding)
  - **adding\_twice** | [https://github.com/01-edu/public/tree/master/subjects/adding\_twice](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/adding_twice)
  - **get\_products** | [https://github.com/01-edu/public/tree/master/subjects/get\_products](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/get_products)
  - **highest** | [https://github.com/01-edu/public/tree/master/subjects/highest](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/highest)
  - **iterators** | [https://github.com/01-edu/public/tree/master/subjects/iterators](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/iterators)
  - **roman\_numbers\_iter** | [https://github.com/01-edu/public/tree/master/subjects/roman\_numbers\_iter](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/roman_numbers_iter)
  - **slices\_to\_map** | [https://github.com/01-edu/public/tree/master/subjects/slices\_to\_map](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/slices_to_map)
  - **step\_iterator** | [https://github.com/01-edu/public/tree/master/subjects/step\_iterator](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/step_iterator)
  - **project\_motion** | [https://github.com/01-edu/public/tree/master/subjects/project\_motion](https://www.google.com/search?q=https://github.com/01-edu/public/tree/master/subjects/project_motion)

### Raid 02-rust

Exercice:

- road-intersection | https://github.com/01-edu/public/tree/master/subjects/road-intersection
