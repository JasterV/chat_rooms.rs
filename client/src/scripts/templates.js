export const leftBubble = (response) => `<div class="bubble-right msg-bubble">
                                            <div class="bubble-header"><i class="fas fa-user-ninja"></i> ${response.id}</div>
                                            ${response.msg}
                                        </div>`;

export const rightBubble = (response) => `<div class="bubble-left msg-bubble">${response.msg}</div>`;