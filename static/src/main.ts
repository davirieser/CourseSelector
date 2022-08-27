
import { Calendar } from '@fullcalendar/core';
import * as $ from 'jquery';

import adaptivePlugin from '@fullcalendar/adaptive';
import interactionPlugin from '@fullcalendar/interaction';
import dayGridPlugin from '@fullcalendar/daygrid';
import listPlugin from '@fullcalendar/list';
import timeGridPlugin from '@fullcalendar/timegrid';
import resourceTimelinePlugin from '@fullcalendar/resource-timeline';

import './main.css';

type Response = {
	success: boolean;
	type: string;
	data: any;
	error?: string;
};

type Faculty = Object;

type Object = {
	id: number;
	name: string;
};

type Course = {
	id: number;
	times: [CourseDate];
};

type CourseDate = {
	date: string;
	time: string;
	location: string;
	comment: string;
}

let selectorEl = $('#courseSelector')!;
let calendarEl: HTMLElement = document.getElementById('calendar')!;
let calendar : Calendar;

document.addEventListener('DOMContentLoaded', function() {
	// Initialize Calendar
	calendar = new Calendar(calendarEl, {
		plugins: [
			adaptivePlugin, interactionPlugin, dayGridPlugin,
			listPlugin, timeGridPlugin, resourceTimelinePlugin
		],
		// now: '2018-02-07',
		editable: false,
		headerToolbar: {
			left: 'today prev,next',
			center: 'title',
			right: 'resourceTimelineDay,timeGridWeek,dayGridMonth,listWeek,listYear'
		},
		buttonText: {
			today: 'Jump to Today',
			prev: 'Previous',
			next: 'Next',
			day: 'Day',
			week: 'Week',
			month: 'Month',
			listWeek: 'List Week',
			listYear: 'List Year'
		},
		schedulerLicenseKey: 'GPL-My-Project-Is-Open-Source',
		slotEventOverlap: false,
		allDaySlot: false,
		slotDuration: { minutes: 15 },
		slotLabelInterval: { hours: 1 },
		slotMinTime: { hours: 8 },
		scrollTime: { hours: 8 },
		scrollTimeReset: false,
		slotMaxTime: { hours: 20 },
		initialView: 'timeGridWeek',
		resourceAreaHeaderContent: 'Rooms',
		resources: [],
		events: []
	});

	calendar.render();

	get_faculties((e: Response) => {
		if (e.success) {
			selectorEl.append(
				create_object_list(e.type, e.data as [Faculty])
			);
		} else {
			alert("Error fetching Faculties")
		}
	});


});

function expand(elem: JQuery<HTMLElement>) {
	switch (elem.attr('data-type')) {
		case 'Object':
			get_lfu_objects(elem.data('data-id') , (e: Response)=> {
				selectorEl.append(create_object_list(e.type, e.data as [Object]))
			})
			break;
		case 'CourseDetails':
			get_course(elem.data('data-id'), (e: Response) => {
				console.log(e)
				addCourseToCalendar(elem.text(), e as unknown as Course)
			})
			break;
		default:
			alert(`Unknown Type "${elem.attr('data-type')}" - Cannot expand`)
			break;
	}
}

function create_object_list(type: string, data: [Object]) : JQuery<HTMLElement> {
	let list = $('<ul></ul>');
	for (let s of data) {
		let item = $(`<li data-id="${s.id}" data-type="${type}">${s.name}</li>`);
		item.data('data-id', s.id);
		item.data('data-type', type);
		item.click(() => { expand(item); });
		list.append(item);
	}
	return list;
}

function get_faculties(callback: (r: Response) => void) {
	get_lfu("/lfu", callback);
}

function get_course(id: number, callback: (r: Response) => void) {
	get_lfu(`/course/${id}`, callback);
}

function get_lfu_objects(id: number, callback: (r: Response) => void) {
	get_lfu(`/lfu/${id}`, callback)
}

function get_lfu(url: string, callback: (r: Response) => void) {
	$.ajax({
		method: "GET",
		dataType: "json",
		url: url,
		success: callback,
		error: () => {
			alert(`Error requesting ${url}`)
		}
	})
}

function addCourseToCalendar(name: string, c: Course) {
	for (let date of c.times) {
		let timeSplit= date.time.split('-');

		let startSplit = timeSplit[0].trim().split('.');
		let startHour = +startSplit[0], startMinute = +startSplit[1];

		let endSplit = timeSplit[0].trim().split('.');
		let endHour = +endSplit[0], endMinute = +endSplit[1];

		let split = date.date.split('.');
		let year = +split[2], month = +split[1], day = +split[0];
		let start = new Date(year, month - 1, day, startHour, startMinute);
		let end = new Date(year, month - 1, day, endHour, endMinute);

		calendar.addEvent({
			start: start,
			end: end,
			title: name + '\n' + date.location + '\n' + date.comment,
			groupId: "" + c.id
		})
	}
}
