/*
 * The logic of the randomiser is an iterative process:
 *
 * The logic for the locations and checks is evaluated based on the checks
 * The available checks are populated at random
 * If more locations/checks are added then the check(s) that added them are marked as progression
 * All the items not marked are purged from the list
 *
 * This loop repeats until all locations and checks are now accessible :)
 * After that we'll have a fully populated Dictionary<string,Item>
 * The generated seed data can then be written appropriately
 */
fn gen_seed() {}
