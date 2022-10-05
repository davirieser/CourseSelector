
import { error } from "@sveltejs/kit";

export async function getLFU(){
    const res = await fetch('http://127.0.0.1:8080/lfu');
    const res2 = await res.json() ;

    if(res.ok) return res2;
    throw error(res2.message);

}
   
export async function getLFUID(id: number){
    const res = await fetch(`http://127.0.0.1:8080/lfu/${id}`);
    const res2 = await res.json();
    if(res.ok) return res2;
    throw error(res2.message);
}


export async function getCourse(id: number, name: string){
    const res = await fetch(`http://127.0.0.1:8080/course/${id}`);
    const res2 = await res.json();
    let body = {
        id: res2.id,
        times: [{
            newGroup: false,
            group: 0,
            date: "",
            time: "",
            start: "",
            end: "",
            title: "",
            location: "",
            comment: ""
        }]
    }
    let groupID = 0;
    let groupIDthreshold = 0;
    let i = 0;
    let newGroup = false;
    for(let date of res2.times){
       
        let timeSplit= date.time.split('-');

        let startSplit = timeSplit[0].trim().split('.');
        let startHour = +startSplit[0], startMinute = +startSplit[1];

        let endSplit = timeSplit[0].trim().split('.');
        let endHour = +endSplit[0], endMinute = +endSplit[1];

        let split = date.date.split('.');
        let year = +split[2], month = +split[1], day = +split[0];
        let start = new Date(year, month - 1, day, startHour, startMinute);
        let end = new Date(year, month - 1, day, endHour, endMinute);
        let title = name + '\n' + date.location + '\n' + date.comment;
        if((date.date == "" && date.time == "" && date.location == "" && date.comment == "") || i===0){
            groupIDthreshold++;
            if((groupIDthreshold == 3) || i===0){
                groupID++;
                groupIDthreshold = 0;
                body.times.push({
                    newGroup: false,
                    group: groupID,
                    date: "---",
                    time: "---",
                    start: "---",
                    end: "---",
                    title: "---",
                    location: "---",
                    comment: "---",
                })
                newGroup=true;
            }
        }else{
            if(newGroup){
                body.times.push({
                    newGroup: newGroup,
                    group: groupID,
                    date: date.date,
                    time: date.time,
                    start: JSON.stringify(start),
                    end: JSON.stringify(end),
                    title: title,
                    location: date.location,
                    comment: date.comment
                })
                newGroup = false;
            }else{
                body.times.push({
                    newGroup: false,
                    group: groupID,
                    date: date.date,
                    time: date.time,
                    start: JSON.stringify(start),
                    end: JSON.stringify(end),
                    title: title,
                    location: date.location,
                    comment: date.comment
                })
            }
            
        }
        i++;
    }
     
    if(res.ok) return body;
    throw error(res2.message);
}


  