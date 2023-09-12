<template>
  <v-container class="fill-height">
    <v-btn @click="start">Start</v-btn>
    <v-btn @click="join" :disabled="meetingAvalible">Join</v-btn>
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
import { FragmentType, graphql, useFragment } from "@/gql";
import { RTCWS } from "@/ws";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { onMounted, reactive } from "vue";
import { computed } from "vue";
import { ref, watch } from "vue";

const MeQuery = graphql(/* GraphQL */ `
  query MeetingMeQuery {
    me {
      id
    }
  }
`);

const { result: meResult } = useQuery(MeQuery);
const myId = computed(() => meResult.value?.me.id ?? "");

const MeetingFragment = graphql(/* GraphQL */ `
  fragment MeetingFragment on Class {
    id
    ownerId
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof MeetingFragment> | null;
  loading: boolean;
}>();

const class_ = computed(() => useFragment(MeetingFragment, props.class_));

let localstream: MediaStream | null = null;
// let remoteStream: MediaStream | null = null;
// let peerConnection: RTCPeerConnection;
let meetingAvalible = ref(true);

const streams = reactive(new Map<string, MediaStream>());
const connectedPeers = new Map<string, RTCPeerConnection>();

const token = localStorage.getItem("token");
const ws = new RTCWS("ws://localhost:3000/rtc-ws");

ws.auth(token!);
ws.subscribe(class_.value!.id);

ws.meetingStartedHandler = async () => {
  meetingAvalible.value = false;
};

ws.userJoinedHandler = async (data) => {
  if (data.user_id === myId.value) {
    return;
  }
  streams.set(data.user_id, new MediaStream());
  await createOffer(data.user_id);
};

ws.offerHandler = async (data) => {
  await createAnswer(data.sender_id, data.offer);
};

ws.answerHandler = async (data) => {
  const peerConnection = connectedPeers.get(data.sender_id)!;
  await peerConnection.setRemoteDescription(data.answer);
};

ws.iceCandidateHandler = async (data) => {
  const peerConnection = connectedPeers.get(data.sender_id)!;
  await peerConnection.addIceCandidate(data.candidate);
};

const start = async () => {
  localstream = await navigator.mediaDevices.getUserMedia({
    video: true,
    audio: false,
  });
  streams.set(myId.value, localstream);

  ws.startMeeting(class_.value!.id);
};
const join = async () => {
  localstream = await navigator.mediaDevices.getUserMedia({
    video: true,
    audio: false,
  });
  streams.set(myId.value, localstream);
  ws.joinMeeting(class_.value!.id);
};

const createOffer = async (targetUserId: string) => {
  const peerConnection = await createPeerConnection(targetUserId);
  let offer = await peerConnection.createOffer();
  await peerConnection.setLocalDescription(offer);
  ws.sendOffer(targetUserId, class_.value!.id, offer);
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
  ws.sendAnswer(targetUserId, class_.value!.id, answer);
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
    ws.sendIceCandidate(targetUserId, class_.value!.id, event.candidate);
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
