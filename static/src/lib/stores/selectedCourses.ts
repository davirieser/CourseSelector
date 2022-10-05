import { writable } from "svelte/store";
import { browser } from "$app/environment";

export const selectedCourses = writable(browser && (localStorage.getItem("selectedCourses")) || []);

selectedCourses.subscribe((value) => {
    if (browser) {
        localStorage.setItem("selectedCourses", value);
    }
});