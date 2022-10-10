<script lang="ts">
    import FullCalendar, { type CalendarOptions } from 'svelte-fullcalendar';
    import adaptivePlugin from '@fullcalendar/adaptive';
    import interactionPlugin from '@fullcalendar/interaction';
    import daygridPlugin from '@fullcalendar/daygrid';
    import listPlugin from '@fullcalendar/list';
    import timegridPlugin from '@fullcalendar/timegrid';
    import resourceTimelinePlugin from '@fullcalendar/resource-timeline';

    let events: any[] = [];
    let options: CalendarOptions = {
      initialView: 'timeGridWeek',
      height: "auto",
      plugins: [
        adaptivePlugin, interactionPlugin, daygridPlugin,
        listPlugin, timegridPlugin, resourceTimelinePlugin
      ],
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
      slotEventOverlap: false,
      allDaySlot: false,
      slotDuration: { minutes: 15 },
      slotLabelInterval: { hours: 1 },
      slotMinTime: { hours: 8 },
      scrollTime: { hours: 8 },
      scrollTimeReset: false,
      slotMaxTime: { hours: 20 },
      schedulerLicenseKey: 'GPL-My-Project-Is-Open-Source',
      resourceAreaHeaderContent: 'Rooms',
      resources: [],
      events
    };



    import {getLFU, getLFUID } from "./+page";
    import Spinner from "$components/spinner.svelte";
    import Select from '$components/select.svelte';
	  import Accordion from '$components/accordion.svelte';


    let selectedFaculty = "";
    let selectedFacultyID = 0;
    $: selectedFacultyID = parseInt(selectedFaculty.split(" ")[0], 10);

    let selectedCurriculum = "";
    let selectedCurriculumID = 0;
    $: selectedCurriculumID = parseInt(selectedCurriculum.split(" ")[0], 10);

    let selectedCourseCategory = "";
    let selectedCourseCategoryID = 0;
    $: selectedCourseCategoryID = parseInt(selectedCourseCategory.split(" ")[0], 10);

    import { selectedCoursesStore } from "$lib/stores/selectedCourses";

    let hideCourses = false;
    let courses = $selectedCoursesStore
    $: console.log($selectedCoursesStore);
    $: {
      let localEvents = [];
      if(courses.length > 0){
        for(let course of courses){
          console.log(course);
          let start = course.split(",")[0];
          let end = course.split(",")[1];
          let title = course.split(",")[2];
          let group = course.split(",")[3];
          let event = {
            start: new Date(JSON.parse(start)),
            end: new Date(JSON.parse(end)),
            title: title,
            group: group
          };
          // console.log(event);
          localEvents.push(event);
        }
        // console.log("LocalEvents ");
        // console.log(localEvents);
        options = {
          ...options, 
          events: localEvents
        }
      }
    }
    // $: {
    //   console.log("Events "); 
    //   console.log(events);
    // }
</script>


<main class="m-5">
    <h1 class="underline text-4xl justify-center mx-0 flex mb-12">Course Selector</h1>
    <div class="grid grid-cols-3 gap-2">
      {#await getLFU()}
        <div class="flex justify-center mx-auto">
          <Spinner />
        </div>
      {:then data}
        <Select bind:bindto={selectedFaculty} data={data} text="Select Faculty" />
      {:catch error}
        <p>{error}</p>
      {/await}

      
      {#if selectedFaculty.length > 1}
        {#await getLFUID(selectedFacultyID)}
          <div class="flex justify-center mx-auto">
            <Spinner />
          </div>
        {:then data}
          <Select bind:bindto = {selectedCurriculum} data={data} text="Select Curriculum" />
        {/await}
      {/if}


      {#if selectedCurriculum.length > 1}
        {#await getLFUID(selectedCurriculumID)}
          <div class="flex justify-center mx-auto">
            <Spinner />
          </div>
        {:then data}
          <Select bind:bindto = {selectedCourseCategory} data={data} text="Select course type" />
        {/await}
      {/if}

      
    </div>  

    <br class="h-5"/>
    {#if selectedCourseCategory.length > 1}
      <div class="divider"><button on:click={()=>hideCourses = !hideCourses} class="btn {hideCourses ? "btn-secondary" : "btn-primary"}">Hide Courses</button></div>
      <div class="{hideCourses ? "hidden" : "block"}">
          {#await getLFUID(selectedCourseCategoryID)}
            <div class="flex justify-center mx-auto">
              <Spinner />
            </div>
          {:then data}
            {#each data.data as data}
              <div class="p-2">
                <Accordion title={data.name.slice(0, 80)} content={data.name} courseVarationID={data.id}/>
              </div>
            {/each}
          {/await}
      </div>
    {/if}

    <div id="button">
      <FullCalendar {options} />
    </div>
</main>
