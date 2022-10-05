<script lang="ts">
	import Spinner from '$components/spinner.svelte';
    import { getCourse } from "$src/routes/+page";
    import { selectedCourses } from "$lib/stores/selectedCourses";
    export let selectedCourseVarationID = 0;
    export let selectedCourseName = "";
    $: console.log($selectedCourses);

    let localCourse = "";
    $: console.log(localCourse);
</script>

{#await getCourse(selectedCourseVarationID, selectedCourseName)}
    <div class="flex justify-center mx-auto">
        <Spinner />
    </div>
{:then data} 
    <div class="w-auto mx-auto justify-center flex">
        <table class="table w-auto mx-auto">
            <thead>
                <tr>
                    <th>Group</th>
                    <th>Date</th>
                    <th>Time</th>
                    <th>Location</th>

                </tr>
            </thead>
            <tbody>
                {#each data.times as data, i}
                    <tr>
                        <th>{data.group}</th>
                        <td>{data.date}</td>
                        <td>{data.time}</td>
                        <td>{data.location}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
        <div class="ml-2">
            <button class="btn btn-primary">{selectedCourseName}</button>
        </div>
    </div>
{/await}