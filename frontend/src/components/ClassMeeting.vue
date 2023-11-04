<template>
  <v-toolbar dark>
    <div v-if="isOwner">
      <v-btn @click="start">Start</v-btn>
      <v-btn @click="stop">Stop</v-btn>
    </div>
    <div v-else>
      <v-btn @click="join" :disabled="!meetingAvalible">Join</v-btn>
      <v-btn @click="leave">Leave</v-btn>
    </div>
  </v-toolbar>
  <v-container class="fill-height">
    <div class="videos">
      <video
        v-for="stream in streams.values()"
        :srcObject="stream"
        class="player"
        autoplay
        playsinline
      ></video>
    </div>
  </v-container>
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

let localstream: MediaStream | null = null;
const meetingAvalible = ref(false);
const meetingRoom = props.meetingRoom;
const streams = reactive(new Map<string, MediaStream>());
const connectedPeers = reactive(new Map<string, RTCPeerConnection>());

onMounted(async () => {
  const meetingData = await meetingRoom.getCurrentMeeting();
  if (meetingData?.peer_ids.length > 0) {
    meetingAvalible.value = true;
    console.log(meetingData);
  }
});

meetingRoom.onMeetingStarted = async () => {
  meetingAvalible.value = true;
};

meetingRoom.onMeetingStopped = async () => {
  meetingAvalible.value = false;
  connectedPeers.clear();
  streams.clear();
};

meetingRoom.onUserJoined = async (data) => {
  if (data.user_id === myId.value) {
    return;
  }
  if (!streams.has(myId.value)) {
    return;
  }

  streams.set(data.user_id, new MediaStream());
  await createOffer(data.user_id);
};

meetingRoom.onUserLeft = async (data) => {
  streams.delete(data.user_id);
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
  const peerConnection = connectedPeers.get(data.sender_id)!;
  await peerConnection.addIceCandidate(data.candidate);
};

const start = async () => {
  localstream = await navigator.mediaDevices.getUserMedia({
    video: true,
    audio: false,
  });
  streams.set(myId.value, localstream);

  meetingRoom.startMeeting();
};

const stop = async () => {
  meetingRoom.stopMeeting();
  streams.clear();
  connectedPeers.clear();
};

const join = async () => {
  localstream = await navigator.mediaDevices.getUserMedia({
    video: true,
    audio: false,
  });
  streams.set(myId.value, localstream);
  meetingRoom.joinMeeting();
};

const leave = async () => {
  meetingRoom.leaveMeeting();
  streams.clear();
  connectedPeers.clear();
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
  console.time("createAnswer");
  const peerConnection = await createPeerConnection(targetUserId);
  await peerConnection.setRemoteDescription(offer);
  const answer = await peerConnection.createAnswer();
  await peerConnection.setLocalDescription(answer);
  meetingRoom.sendAnswer(targetUserId, answer);
  connectedPeers.set(targetUserId, peerConnection);
  console.log(connectedPeers);
  console.timeEnd("createAnswer");
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
</script>

<style scoped>
.videos {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2em;
}

.player {
  width: 100%;
  height: 300px;
  background-color: black;
}
</style>
