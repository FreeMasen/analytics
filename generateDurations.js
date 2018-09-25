const moment = require('moment');
function main() {
    for (var i = 0; i < 100; i++) {
        console.log(generateDuration().toString()``);
    }
}

function generateDuration() {
    let now = moment();
    let year = random(1970, now.year());
    let month = random(0, 12);
    let maxDay = getMaxDay(month);
    let day = random(1, maxDay);
    let start = moment([
        year,
        month,
        day,
    ]);
    return moment.duration(now.diff(start))
}

function getMaxDay(month) {
    switch (month) {
        case 0:
        case 2:
        case 4:
        case 6:
        case 7:
        case 9:
        case 11:
            return 31
        case 1:
            return 28
        default:
            return 30
    }
}

function random(min, max) {
    let multiplier = max - min;
    return Math.floor(Math.random() * multiplier) + min
}

main();