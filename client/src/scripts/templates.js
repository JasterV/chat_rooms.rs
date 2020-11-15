export const leftBubble = (response) => `<div class="bubble-right msg-bubble">
                                            <div class="bubble-header"><i class="fas fa-user-ninja"></i> ${response.id}</div>
                                            ${response.msg}
                                        </div>`;

export const rightBubble = (response) => `<div class="bubble-left msg-bubble">${response.msg}</div>`;

export const newUserText = (response) => `<div class="info-msg">A new <i class="fas fa-user-ninja"></i> has joined the room!</div>`;

export const userGoneText = () => `<div class="info-msg">A <i class="fas fa-user-ninja"></i> has left the room</div>`;