import * as moment from 'moment';
const now = moment();
const COOKIE_KEY = 'pizzalitics';
const VISIT_KEY = 'slice';
const method = 'POST';
let exit_link;
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
    exit_link = (ev.currentTarget as HTMLAnchorElement).href;
}
/**
 * Send the landing info to the analytics server
 * @param url The full url for the endpoint to hit
 */
export function sendInfo(url = '/analytics/landing', info: LandingInfo = new LandingInfo()): Promise<InitialResponse> {
    let reqInit = {
        method,
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(info),
    };
    return fetch(url, reqInit).then(r => {
        if (!r.ok) {
            return Promise.reject('Failed to send info: ' + r.statusText);
        } else {
            return r.json()
        }
    })
}
/**
 * Store the initial response from the web server
 * @param r The initial response from the web server
 */
export function initialResponseHandler(r: InitialResponse) {
    console.log('storing token', r);
    localStorage.setItem(COOKIE_KEY, r.token);
    localStorage.setItem(VISIT_KEY, r.visit);
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
export class LandingInfo {
    constructor(
        public referrer = safeString(document.referrer),
        public page = document.location.href,
        public cookie = safeString(localStorage.getItem(COOKIE_KEY)),
        public when = moment.utc(),
        public prev_visit = safeString(localStorage.getItem(VISIT_KEY)),
    ) {}
}

function safeString(str): string | null {
    if (!str) return null;
    if (str === '') return null;
    return str;
}

/**
 * Send the last information about this user browsing this
 * page
 * @param url The endpoint to send the exiting info to
 */
export function sendExiting(url = '/analytics/exiting', info: ExitingInfo = new ExitingInfo()) {
    let xhr = new XMLHttpRequest();
    xhr.open(method, url, false);
    xhr.setRequestHeader('Accept', 'application/json');
    xhr.setRequestHeader('Content-Type', 'application/json');
    let body = JSON.stringify(info);
    xhr.send(body);
}

/**
 * The information captured as a user is exiting
 * the page
 */
export class ExitingInfo {
    constructor(
        public visit: string = safeString(localStorage.getItem(VISIT_KEY)),
        public time: moment.Duration = moment.duration(moment().diff(now)),
        public link_clicked: string = exit_link,
        ) {}

    toJSON(): any {
        return {
            visit: this.visit,
            time: this.time.toString(),
            link_clicked: this.link_clicked,
        }
    }
}