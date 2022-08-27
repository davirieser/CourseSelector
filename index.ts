import { Calendar } from '@fullcalendar/core';
import dayGridPlugin from '@fullcalendar/daygrid';

type LfuObject = { id: String, text?: String };
type ServerResponse = { success: boolean, result: LfuObject[] };

$(document).ready(() => {
    const calendar_element = $('#calendar').get()[0];
    const calendar : Calendar = new Calendar(calendar_element, {
        plugins: [ dayGridPlugin ]
    });
    calendar.render();
    $.ajax({
        url: '/lfu/asdf',
        method: "GET",
        success: (e : ServerResponse) => {
            if (e.success) {
                console.log(`Got Faculties: ${e.result.map((o) => o.text)}`);
            } else {
                console.log("Could not get facilities");
            }
        },
        error: (e) => {
            console.log("Could not get facilities");
        }
    })
})

