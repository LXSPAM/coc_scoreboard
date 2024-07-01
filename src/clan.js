const { invoke } = window.__TAURI__.tauri;
const { emit } = window.__TAURI__.event;
let isScoreBoardOpen = false;

function back() {
  window.history.back();
  invoke('adjust_window_size', { width: 550, height: 370 });
  invoke('close_window', {});
}

function getQueryParams() {
  const params = {};
  window.location.search.substring(1).split('&').forEach(pair => {
    const [key, value] = pair.split('=');
    params[key] = decodeURIComponent(value);
  });
  return params;
}

function formatDuration(seconds) {
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;
  return `${minutes}m${remainingSeconds}s`;
}

async function initPage() {
  const queryParams = getQueryParams();
  const clanTag = queryParams.tag;
  document.title = `Clan - ${clanTag}`;
  await updateClanWar(clanTag);
  setInterval(() => updateClanWar(clanTag), 1500);  // Update every 10 seconds
  resizeWindowToFitContent();
  toggleScoreBoard(clanTag);
}

async function updateClanWar(clanTag) {
  const data = await getClanWar(clanTag);
  if (data) {
    if (data.state != 'notInWar') {let string_data = JSON.stringify(data);
    invoke('parse_wardata', { data:string_data }).then((response) => {
      if (response.status === 200) {
        let rep_data = response.message;
        emit('scoreboardUpdate', { rep_data });
      } else {
        console.error(response.message);
      }
    });}
    populateClanData(data);
  }
}

async function getClanWar(clanTag) {
  return invoke('get_war', { tag: clanTag })
    .then((response) => {
      if (response.status === 200) {
        return JSON.parse(response.message);
      } else if (response.status === 404) {
        return 404;
      } else {
        throw new Error(response.message);
      }
    })
    .catch((error) => {
      console.error(error);
    });
}

function populateClanData(data) {
  const warStatus = document.getElementById('war-status');
  
  if (data.state === "notInWar") {
    warStatus.innerHTML = `
      <h2>War state</h2>
      <p>전쟁중이 아님. 대기중.. 클랜전 시작시 자동으로 바뀜</p>
    `;
    document.getElementById('time').innerHTML = "";
    document.getElementById('time_difference').innerHTML = "";
  }
  
  document.getElementById('clan').innerHTML = `
    <img src="${data.clan.badgeUrls.large}" alt="Badge">
    <p style="font-size: larger;">${data.clan.name}</p>
  `;
  
  document.getElementById('opponent').innerHTML = `
    <img src="${data.opponent.badgeUrls.large}" alt="Badge">
    <p style="font-size: larger;">${data.opponent.name}</p>
  `;
  
  let clanTotalTime = 0;
  let opponentTotalTime = 0;
  let clanTotalpercentage = 0;
  let opponentTotalpercentage = 0;
  
  document.getElementById('members-left').innerHTML = '';
  document.getElementById('members-right').innerHTML = '';
  
  function addMembers(members, containerId) {
    let container = document.getElementById(containerId);
    let subContainer;
    members.forEach((member, index) => {
      if (index % 10 === 0) {
        subContainer = document.createElement('div');
        subContainer.classList.add('sub-container');
        container.appendChild(subContainer);
      }
      subContainer.innerHTML += `
        <div class="member">
          <div class="name">
            <p>${member.name}</p>
          </div>
          ${member.attacks ? `
            <div class="stats">
              <p>${member.attacks[0].stars}★ ${member.attacks[0].destructionPercentage}% ${formatDuration(member.attacks[0].duration)}</p>
            </div>
          ` : ''}
        </div>
      `;
    });
  }
  if (data.state !== "notInWar") {
  addMembers(data.clan.members, 'members-left');
  addMembers(data.opponent.members, 'members-right');
  
  data.clan.members.forEach(member => {
    if (member.attacks) {
      clanTotalTime += member.attacks[0].duration;
      clanTotalpercentage += member.attacks[0].destructionPercentage;
    }
  });
  
  data.opponent.members.forEach(member => {
    if (member.attacks) {
      opponentTotalTime += member.attacks[0].duration;
      opponentTotalpercentage += member.attacks[0].destructionPercentage;
    }
  });
  
  document.getElementById('time').innerHTML = `
    <p>${clanTotalpercentage}% ${formatDuration(clanTotalTime)} VS ${opponentTotalpercentage}% ${formatDuration(opponentTotalTime)}</p>

  `;
  
    document.getElementById('time_difference').innerHTML = `
      <p>${Math.abs(clanTotalpercentage - opponentTotalpercentage)}% ${formatDuration(clanTotalTime - opponentTotalTime)} 차이남</p>
    `;
  
}
  const stateText = {
    preparation: '준비중',
    inWar: '전쟁중',
    warEnded: '전쟁종료. 다음전쟁까지 대기',
    default: '전쟁중이 아님. 대기중..'
  };
  
  warStatus.innerHTML = `
    <h2>War state</h2>
    <p>${stateText[data.state] || stateText.default}</p>
  `;
}

function resizeWindowToFitContent() {
  const contentHeight = document.body.scrollHeight;
  const contentWidth = document.body.scrollWidth; 
  invoke('adjust_window_size', { width: contentWidth, height: contentHeight })
    .catch((error) => {
      console.error('Error resizing window:', error);
    });
}

async function toggleScoreBoard(clanTag) {
  const data = await getClanWar(clanTag);
  const url = `scoreboard.html?war_data=${encodeURIComponent(JSON.stringify(data))}`;
  const width = 320;
  const height = 391;
  const transparent = true;
  const decorations = false;
  const name = 'scoreboard';

  invoke('open_new_window', { 
    url: url,
    width: width,
    height: height,
    transparent: transparent,
    decorations: decorations,
    name: name
  })
  .catch((error) => {
    console.error('Error opening new window:', error);
  });
}

function update_logo() {
  const clan_default_logo = document.getElementById('clan_default_logo').checked;
  const opponent_default_logo = document.getElementById('opponent_default_logo').checked;
  const clan_custom_logo = document.getElementById('clan_custom_logo').files[0];
  const opponent_custom_logo = document.getElementById('opponent_custom_logo').files[0];

  const reader1 = new FileReader();
  const reader2 = new FileReader();
  
  let clan_logo_base64 = null;
  let opponent_logo_base64 = null;

  reader1.onload = () => {
    clan_logo_base64 = reader1.result.split(',')[1];
    emitLogoupdate();
  };
  
  reader2.onload = () => {
    opponent_logo_base64 = reader2.result.split(',')[1];
    emitLogoupdate();
  };

  if (clan_custom_logo) {
    reader1.readAsDataURL(clan_custom_logo);
  } else {
    emitLogoupdate();
  }

  if (opponent_custom_logo) {
    reader2.readAsDataURL(opponent_custom_logo);
  } else {
    emitLogoupdate();
  }

  function emitLogoupdate() {
    
        emit('logoupdate', { 
          logo_option1: clan_default_logo, 
          logo_option2: opponent_default_logo, 
          logo1_file: clan_logo_base64, 
          logo2_file: opponent_logo_base64 
        });
      }
    
  }
