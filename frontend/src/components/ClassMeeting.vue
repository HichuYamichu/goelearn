<template>
  <v-toolbar dark>
    <div class="ml-4" v-if="isOwner">
      <v-btn @click="start" :disabled="meetingAvalible">Start</v-btn>
      <v-btn @click="stop" :disabled="!meetingAvalible">Stop</v-btn>
    </div>
    <div class="ml-4" v-else>
      <v-btn @click="join" :disabled="!meetingAvalible || userIsInMeeting"
        >Join</v-btn
      >
      <v-btn @click="leave" :disabled="!userIsInMeeting">Leave</v-btn>
    </div>
    <v-spacer></v-spacer>
    <div class="mr-4" v-if="userIsInMeeting">
      <v-btn :icon="screanShareIcon" @click="toggleScreanShare"></v-btn>
      <v-btn :icon="microphoneIcon" @click="toggleMic"></v-btn>
    </div>
    <v-menu>
      <template v-slot:activator="{ props }">
        <v-btn icon="mdi-dots-vertical" v-bind="props"></v-btn>
      </template>
      <v-card>
        <v-card-item>
          <v-card-title>Pick your camera</v-card-title>
          <v-list>
            <v-list-item
              v-for="(item, index) in avalibleCam"
              :key="index"
              :value="index"
              @click="changeCam(index)"
            >
              <v-list-item-title>{{ item.label }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-card-item>
        <v-card-item>
          <v-card-title>Pick your microphone</v-card-title>
          <v-list>
            <v-list-item
              v-for="(item, index) in avalibleMic"
              :key="index"
              :value="index"
              @click="changeMic(index)"
            >
              <v-list-item-title>{{ item.label }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-card-item>
      </v-card>
    </v-menu>
  </v-toolbar>
  <div class="solo-container" v-if="focusedUser">
    <video
      class="video-player"
      :srcObject="focusedUser"
      @click="focusedUserId = null"
      autoplay
      playsinline
    ></video>
  </div>
  <div v-else>
    <v-container class="fill-height justify-center max-height">
      <div class="video-grid">
        <div class="video-container">
          <video
            class="video-player"
            :srcObject="localMediaStream"
            autoplay
            playsinline
            muted
          ></video>
        </div>
        <div
          class="video-container"
          v-for="[id, stream] in nonLocalStreams"
          @click="focusedUserId = id"
        >
          <video
            class="video-player"
            :srcObject="stream"
            autoplay
            playsinline
            :muted="id === myId"
          ></video>
        </div>
      </div>
    </v-container>
  </div>
</template>

<script lang="ts" setup>
import { ClassMeetingWS } from "@/class-meeting";
import { FragmentType, graphql, useFragment } from "@/gql";
import { MyIdQuery } from "@/shared";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { onMounted, reactive } from "vue";
import { computed } from "vue";
import { ref, watch } from "vue";

const { result: meResult } = useQuery(MyIdQuery);
const myId = computed(() => meResult.value?.me.id ?? "");

const MeetingFragment = graphql(/* GraphQL */ `
  fragment MeetingFragment on Class {
    id
    ownerId
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof MeetingFragment> | null;
  meetingRoom: ClassMeetingWS;
}>();

const class_ = computed(() => useFragment(MeetingFragment, props.class_));
const isOwner = computed(() => class_.value?.ownerId === myId.value);

const videoConstraints = reactive({
  width: { min: 640, ideal: 1300, max: 1300 },
  height: { min: 480, ideal: 731, max: 731 },
});
const localMediaStream = ref<MediaStream | null>(null);

const meetingAvalible = ref(false);
const userIsInMeeting = ref(false);
const meetingRoom = props.meetingRoom;
const streams = reactive(new Map<string, MediaStream>());
const connectedPeers = reactive(new Map<string, RTCPeerConnection>());
const focusedUserId = ref<string | null>(null);
const focusedUser = computed(() => {
  if (!focusedUserId.value) {
    return null;
  }
  return streams.get(focusedUserId.value);
});
const nonLocalStreams = computed(() => {
  return Array.from(streams.entries()).filter(([id, _]) => id !== myId.value);
});

const microphone = ref(true);
const microphoneIcon = computed(() => {
  return microphone.value ? "mdi-microphone" : "mdi-microphone-off";
});
const screanShare = ref(false);
const screanShareIcon = computed(() => {
  return screanShare.value ? "mdi-monitor-share" : "mdi-monitor-off";
});

const devices = ref<MediaDeviceInfo[]>([]);
const avalibleMic = computed(() => {
  return devices.value.filter((d) => d.kind === "audioinput");
});
const avalibleCam = computed(() => {
  return devices.value.filter((d) => d.kind === "videoinput");
});
const selectedMic = ref(0);
const selectedCam = ref(0);

const resetState = () => {
  console.log(streams);
  console.log(connectedPeers);
  meetingAvalible.value = false;
  userIsInMeeting.value = false;
  streams.clear();
  connectedPeers.clear();
  focusedUserId.value = null;
  microphone.value = true;
  screanShare.value = false;
  localMediaStream.value = null;
};

onMounted(async () => {
  const meetingData = await meetingRoom.getCurrentMeeting();
  if (meetingData?.peer_ids.length > 0) {
    meetingAvalible.value = true;
  }
  devices.value = await navigator.mediaDevices.enumerateDevices();
  console.log(devices.value);
});

meetingRoom.onMeetingStarted = async () => {
  meetingAvalible.value = true;
};

meetingRoom.onMeetingStopped = async () => {
  for (let peerConnection of connectedPeers.values()) {
    peerConnection.close();
  }
  resetState();
};

meetingRoom.onUserJoined = async (data) => {
  if (data.user_id === myId.value) {
    return;
  }
  if (!streams.has(myId.value)) {
    return;
  }
  await createOffer(data.user_id);
};

meetingRoom.onUserLeft = async (data) => {
  streams.delete(data.user_id);
  connectedPeers.get(data.user_id)?.close();
  connectedPeers.delete(data.user_id);
};

meetingRoom.onOffer = async (data) => {
  await createAnswer(data.sender_id, data.offer);
};

meetingRoom.onAnswer = async (data) => {
  const peerConnection = connectedPeers.get(data.sender_id)!;
  await peerConnection.setRemoteDescription(data.answer);
};

meetingRoom.onICECandidate = async (data) => {
  const peerConnection = connectedPeers.get(data.sender_id);
  if (peerConnection) {
    await peerConnection.addIceCandidate(data.candidate);
  }
};

const start = async () => {
  const localstream = await navigator.mediaDevices.getUserMedia({
    video: {
      deviceId: { exact: avalibleCam.value[selectedCam.value].deviceId },
      ...videoConstraints,
    },
    audio: {
      deviceId: { exact: avalibleMic.value[selectedMic.value].deviceId },
    },
  });
  localMediaStream.value = localstream;
  streams.set(myId.value, localstream);
  meetingRoom.startMeeting();
  userIsInMeeting.value = true;
  toggleMic();
};

const stop = async () => {
  meetingRoom.stopMeeting();
  for (let peerConnection of connectedPeers.values()) {
    peerConnection.close();
  }
  resetState();
};

const join = async () => {
  const localstream = await navigator.mediaDevices.getUserMedia({
    video: {
      deviceId: { exact: avalibleCam.value[selectedCam.value].deviceId },
      ...videoConstraints,
    },
    audio: {
      deviceId: { exact: avalibleMic.value[selectedMic.value].deviceId },
    },
  });
  localMediaStream.value = localstream;
  streams.set(myId.value, localstream);
  meetingRoom.joinMeeting();
  userIsInMeeting.value = true;
  toggleMic();
};

const leave = async () => {
  meetingRoom.leaveMeeting();
  for (let peerConnection of connectedPeers.values()) {
    peerConnection.close();
  }
  resetState();
};

const createOffer = async (targetUserId: string) => {
  const peerConnection = await createPeerConnection(targetUserId);
  let offer = await peerConnection.createOffer();
  await peerConnection.setLocalDescription(offer);
  meetingRoom.sendOffer(targetUserId, offer);
  connectedPeers.set(targetUserId, peerConnection);
};

const createAnswer = async (
  targetUserId: string,
  offer: RTCSessionDescriptionInit
) => {
  const peerConnection = await createPeerConnection(targetUserId);
  await peerConnection.setRemoteDescription(offer);
  const answer = await peerConnection.createAnswer();
  await peerConnection.setLocalDescription(answer);
  meetingRoom.sendAnswer(targetUserId, answer);
  connectedPeers.set(targetUserId, peerConnection);
};

const createPeerConnection = async (
  targetUserId: string
): Promise<RTCPeerConnection> => {
  const peerConnection = new RTCPeerConnection({
    iceServers: [
      {
        urls: ["stun:stun.l.google.com:19302"],
      },
    ],
  });
  const remoteStream = new MediaStream();
  streams.set(targetUserId, remoteStream);

  const localstream = streams.get(myId.value)!;
  for (let track of localstream!.getTracks()) {
    peerConnection.addTrack(track, localstream!);
  }

  peerConnection.ontrack = (event) => {
    for (let track of event.streams[0].getTracks()) {
      remoteStream!.addTrack(track);
    }
  };

  peerConnection.onicecandidate = (event) => {
    if (!event.candidate) {
      return;
    }
    meetingRoom.sendIceCandidate(targetUserId, event.candidate);
  };

  return peerConnection;
};

const toggleMic = () => {
  microphone.value = !microphone.value;
  const myStream = streams.get(myId.value)!;
  myStream.getAudioTracks()[0].enabled = microphone.value;
};

const toggleScreanShare = async () => {
  screanShare.value = !screanShare.value;
  if (screanShare.value) {
    const displayMediaOptions = {
      audio: false,
    };
    const displayMedia = await navigator.mediaDevices.getDisplayMedia(
      displayMediaOptions
    );
    const displayStream = displayMedia.getVideoTracks()[0];
    for (let peerConnection of connectedPeers.values()) {
      peerConnection
        .getSenders()
        .find((s) => s.track?.kind === displayStream.kind)
        ?.replaceTrack(displayStream);
    }
    localMediaStream.value = displayMedia;
  } else {
    const camStream = await navigator.mediaDevices.getUserMedia({
      video: {
        deviceId: { exact: avalibleCam.value[selectedCam.value].deviceId },
        ...videoConstraints,
      },
      audio: {
        deviceId: { exact: avalibleMic.value[selectedMic.value].deviceId },
      },
    });
    const camTrack = camStream.getVideoTracks()[0];
    for (let peerConnection of connectedPeers.values()) {
      peerConnection
        .getSenders()
        .find((s) => s.track?.kind === camTrack.kind)
        ?.replaceTrack(camTrack);
    }
    localMediaStream.value = camStream;
  }
};

const changeCam = async (idx: number) => {
  selectedCam.value = idx;
  const camStream = await navigator.mediaDevices.getUserMedia({
    video: {
      deviceId: { exact: avalibleCam.value[selectedCam.value].deviceId },
      ...videoConstraints,
    },
    audio: {
      deviceId: { exact: avalibleMic.value[selectedMic.value].deviceId },
    },
  });
  const camTrack = camStream.getVideoTracks()[0];
  for (let peerConnection of connectedPeers.values()) {
    peerConnection
      .getSenders()
      .find((s) => s.track?.kind === camTrack.kind)
      ?.replaceTrack(camTrack);
  }
  localMediaStream.value = camStream;
};

const changeMic = async (idx: number) => {
  selectedMic.value = idx;
  const micStream = await navigator.mediaDevices.getUserMedia({
    video: {
      deviceId: { exact: avalibleCam.value[selectedCam.value].deviceId },
      ...videoConstraints,
    },
    audio: {
      deviceId: { exact: avalibleMic.value[selectedMic.value].deviceId },
    },
  });
  const micTrack = micStream.getAudioTracks()[0];
  for (let peerConnection of connectedPeers.values()) {
    peerConnection
      .getSenders()
      .find((s) => s.track?.kind === micTrack.kind)
      ?.replaceTrack(micTrack);
  }
  streams.set(myId.value, micStream);
};
</script>

<style scoped>
.videos {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 1em;
}

.player {
  width: 100%;
  height: 300px;
  background-color: black;
}

.box {
  width: 100%;
  height: 300px;
  background-color: black;
}

.gap {
  gap: 1em;
}

.video-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  grid-gap: 16px;
  max-height: 100%;
  overflow: auto;
  display: flex;
  flex-wrap: wrap;
}

.video-container {
  position: relative;
  overflow: hidden;
  border-radius: 8px;
  aspect-ratio: 16/9;
  flex: 1 0 calc(25% - 16px);
  margin-right: 16px;
  margin-bottom: 16px;
}

.video-player {
  width: 100%;
  height: 100%;
  object-fit: cover;
  position: relative;
}

.max-height {
  max-height: 70vh;
}

.solo-container {
  display: flex;
  max-height: 100%;
  overflow: hidden;
}
</style>
