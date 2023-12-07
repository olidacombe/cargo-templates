let year = variable::prompt("Year", "2023", "^202[3-9]$");
let day = variable::prompt("Day", "", "^[0-9]{2}$");
let round = variable::prompt("Round", "1", ["1", "2"]);

variable::set("year", year);
variable::set("day", day);
variable::set("round", round);
variable::set("project-name", "aoc" + year + "-" + day + "-" + round);
