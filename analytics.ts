import * as moment from 'moment';
const now = moment();
const COOKIE_KEY = 'pizzalitics';
let link_clicked;
let visit_key;
/**
 * Setup up a click event on all of the <a> tags that will
 * capture the href before exiting the page
 */
export function setup_click_watcher() {
    let anchors = document.getElementsByTagName('a');

    for (var i = 0; i < anchors.length;i++) {
        anchors[i].addEventListener('click', link_clicked_handler);
    }
}
/**
 *
 * @param ev {ClickEvent<HTMLAnchorElement>} - the click event
 */
function link_clicked_handler(ev: MouseEvent) {
    link_clicked = (ev.currentTarget as HTMLAnchorElement).href;
}
/**
 * Send the landing info to the analytics server
 * @param url The full url for the endpoint to hit
 */
export function sendInfo(url = '/analytics'): Promise<void> {
    return fetch(url, {
        method: 'POST',
        body: landingBody(),
    }).then(r => {
        if (!r.ok) {
            console.error('Failed to send info: ', r.statusText);
        } else {
            r.json().then(initialResponseHandler);
        }
    })
}
/**
 * Store the initial response from the web server
 * @param r The initial response from the web server
 */
function initialResponseHandler(r: InitialResponse) {
    document.cookie[COOKIE_KEY] = r.token;
}
/**
 * The initial response from the  server after
 * sending the landing info
 */
interface InitialResponse {
    token: string;
    visit: string;
}
/**
 * The landing information for the server
 */
class LandingInfo {
    referrer: string;
    page: string;
    cookie: string;
    when: moment.Moment;
    constructor() {
        this.referrer = document.referrer;
        this.page = document.location.href;
        this.cookie = document.cookie[COOKIE_KEY];
        this.when = moment();
    }
}
/**
 * Create the landing info for this page and then
 * JSON.stringify it
 */
function landingBody(): string {
    let obj =  new LandingInfo();
    return JSON.stringify(obj);
}
/**
 * Send the last information about this user browsing this
 * page
 * @param url The endpoint to send the exiting info to
 */
export function sendExiting(url = '/analytics/exiting') {
    fetch(url, {
        method: 'POST',
        body: exitingBody(),
    })
}
/**
 * Create the exiting information and then
 * JSON.stringify it
 */
function exitingBody() {
    let obj = new ExitingInfo();
    return JSON.stringify(obj);
}
/**
 * The information captured as a user is exiting
 * the page
 */
class ExitingInfo {
    cookie: string;
    time: moment.Duration;
    link_clicked: string;
    constructor() {
        this.cookie = document.cookie[COOKIE_KEY];
        this.time = moment.duration(moment().diff(now));
        this.link_clicked = link_clicked;
    }

    toJSON(): any {
        return {
            cookie: this.cookie,
            time: this.time.toString(),
            link_clicked: this.link_clicked,
        }
    }
}