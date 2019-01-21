import * as moment from 'moment';
const DISMISSED_NOTIFICATION = 'dismissed-notification';
const NOTIFICATION_ID = 'cookie-notification';

export function notificationNeeded() {
    let prev = localStorage.getItem(DISMISSED_NOTIFICATION);
    if (!prev || prev == '') {
        return true;
    }
    try {
        let prevTime = moment(prev);
        return !prevTime.isValid()
    } catch (e) {
        return true;
    }
}

export function notifyUser() {
    let existing = document.getElementById(NOTIFICATION_ID);
    if (existing) {
        return;
    }
    document.body.appendChild(createNotification());
}
/**
 * @returns {HTMLDivElement} - A filled in container
 */
function createNotification() {
    let div = createContainer();
    let message = createMessage();
    div.appendChild(message);
    let moreInfoButton = createMoreInfoButton();
    div.appendChild(moreInfoButton);
    let dismissButton = createDismissButton();
    div.appendChild(dismissButton);
    return div;
}

/**
 * @returns {HTMLDivElement} - an empty styled container
 */
function createContainer(): HTMLDivElement {
    let div = document.createElement('div');
    div.setAttribute('class', 'cookie-notification');
    div.setAttribute('id', NOTIFICATION_ID);
    div.style.display = 'grid';
    div.style.gridTemplateColumns = '50% 50%';
    div.style.gridTemplateRows = '75% 25%';
    div.style.gridTemplateAreas = `"message message"
                                   "info    dismiss"`;
    div.style.boxShadow = '1px 1px 1px 1px rgba(0,0,0,0.5)';
    div.style.position = 'absolute';
    div.style.top = 'calc(100% - 105px)';
    div.style.left = 'calc(100% - 255px)';
    div.style.height = '100px';
    div.style.width = '250px';
    div.style.background = 'white';
    div.style.padding = '5px';
    return div;
}

function createMessage(): HTMLSpanElement {
    let message = document.createElement('span');
    message.setAttribute('class', 'notification-message');
    message
        .appendChild(
            document.createTextNode('This site captures a minimal amount of information to provide its author with some usage information.')
        );
    message.style.gridArea = 'message';
    message.style.fontFamily = 'sans-serif';
    return message;
}

function createMoreInfoButton(): HTMLAnchorElement {
    let moreInfoButton = document.createElement('a');
    moreInfoButton.setAttribute('class', 'more-info-button notification-button');
    moreInfoButton.appendChild(document.createTextNode('More Info'));
    moreInfoButton.href = 'https://wiredforge.com/analytics_info';
    moreInfoButton.target = '_blank';
    moreInfoButton.rel = 'noopener';
    moreInfoButton.style.textDecoration = 'none';
    moreInfoButton.style.display = 'block';
    moreInfoButton.style.textAlign = 'center';
    styleButton(moreInfoButton, 'black', 'white', 'info');
    return moreInfoButton;
}

function createDismissButton(): HTMLButtonElement {
    let dismissButton = document.createElement('button');
    dismissButton.setAttribute('class', 'dismiss-button notification-button');
    dismissButton.appendChild(document.createTextNode('Dismiss'));
    dismissButton.addEventListener('click', ev => {
        localStorage.setItem(DISMISSED_NOTIFICATION, moment.utc().toISOString());
        let button = ev.currentTarget as HTMLButtonElement;
        button.parentElement.parentElement.removeChild(button.parentElement);
    });
    styleButton(dismissButton, 'white', 'rgba(0,180,255,0.5)', 'dismiss');
    return dismissButton;
}

function styleButton(button: HTMLButtonElement | HTMLAnchorElement, color: string, background: string, gridArea: string) {
    button.style.color = color;
    button.style.background = background;
    button.style.gridArea = gridArea;
    button.style.position = 'relative';
    button.style.boxShadow = '1px 1px 1px 1px rgba(0,0,0,0.5)';
    button.addEventListener('mouseover', () => {
        button.style.boxShadow = '1px 1px 2px 1px rgba(0,0,0,0.8)';
    });
    button.addEventListener('mouseout', () => {
        button.style.boxShadow = '1px 1px 1px 1px rgba(0,0,0,0.5)';
    });
    button.style.margin = '0 5px';
    button.style.border = 'none';
    button.style.fontWeight = 'bold';
    button.style.fontSize = '12pt';
}