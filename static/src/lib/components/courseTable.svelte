<script lang="ts">
	import Spinner from '$components/spinner.svelte';
    import { getCourse } from "$src/routes/+page";

    export let selectedCourseVarationID = 0;
    export let selectedCourses: any[] = [];
</script>

{#await getCourse(selectedCourseVarationID)}
    <div class="flex justify-center mx-auto">
        <Spinner />
    </div>
{:then data} 
    <div class="w-auto">
        <table class="table w-auto">
            <thead>
                <tr>
                    <th>Group</th>
                    <th>Date</th>
                    <th>Time</th>
                    <th>Location</th>
                    <th>Add to calender</th>

                </tr>
            </thead>
            <tbody>
                {#each data.times as data, i}
                    <tr>
                        <th>{i++}</th>
                        <td>{data.date}</td>
                        <td>{data.time}</td>
                        <td>{data.location}</td>
                        <td><input type=checkbox name="course" value={data.date + "," + data.time + "," + data.location} bind:group={selectedCourses}
                            class="bg-slate-800 rounded focus:ring-blue-800 hover:ring-blue-800"/></td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/await}