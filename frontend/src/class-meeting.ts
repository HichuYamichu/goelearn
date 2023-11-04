type SignalServerEventData =
  | MeetingStartedData
  | MeetingStoppedData
  | UserJoinedData
  | UserLeftData
  | OfferData
  | AnswerData
  | IceCandidateData;

type MeetingStartedData = { type: "MeetingStarted" };
type MeetingStoppedData = { type: "MeetingStopped" };
type UserJoinedData = { type: "UserJoined"; user_id: string };
type OfferData = {
  type: "Offer";
  sender_id: string;
  offer: RTCSessionDescriptionInit;
};
type AnswerData = {
  type: "Answer";
  sender_id: string;
  answer: RTCSessionDescriptionInit;
};
type IceCandidateData = {
  type: "IceCandidate";
  sender_id: string;
  candidate: RTCIceCandidateInit;
};
type UserLeftData = { type: "UserLeft"; user_id: string };

type MeetingData = {
  peer_ids: Array<string>;
};

type SignalServerMessage = {};

export class ClassMeetingWS {
  ws: WebSocket;
  classId: string;

  onMeetingStarted?: (data: MeetingStartedData) => Promise<void>;
  onMeetingStopped?: (data: MeetingStoppedData) => Promise<void>;
  onUserJoined?: (data: UserJoinedData) => Promise<void>;
  onUserLeft?: (data: UserLeftData) => Promise<void>;
  onOffer?: (data: OfferData) => Promise<void>;
  onAnswer?: (data: AnswerData) => Promise<void>;
  onICECandidate?: (data: IceCandidateData) => Promise<void>;

  constructor(classId: string) {
    // TODO: use env variable
    this.ws = new WebSocket("ws://localhost:3000/rtc-ws");
    this.classId = classId;
    const token = localStorage.getItem("token");

    this.ws.addEventListener("open", async (event) => {
      this.ws.send(JSON.stringify({ type: "Auth", token, class_id: classId }));
      this.ws.send(
        JSON.stringify({ type: "Subscribe", target_class_id: classId })
      );
    });

    this.ws.addEventListener("message", async (event) => {
      const data: SignalServerEventData = JSON.parse(event.data);
      if (data.type === "MeetingStarted") {
        await this.onMeetingStarted?.(data);
      } else if (data.type === "MeetingStopped") {
        await this.onMeetingStopped?.(data);
      } else if (data.type === "UserJoined") {
        await this.onUserJoined?.(data);
      } else if (data.type === "UserLeft") {
        await this.onUserLeft?.(data);
      } else if (data.type === "Offer") {
        await this.onOffer?.(data);
      } else if (data.type === "Answer") {
        await this.onAnswer?.(data);
      } else if (data.type === "IceCandidate") {
        await this.onICECandidate?.(data);
      }
    });
  }

  startMeeting() {
    this.ws.send(JSON.stringify({ type: "StartMeeting" }));
  }

  stopMeeting() {
    this.ws.send(JSON.stringify({ type: "StopMeeting" }));
  }

  joinMeeting() {
    this.ws.send(JSON.stringify({ type: "JoinMeeting" }));
  }

  leaveMeeting() {
    this.ws.send(JSON.stringify({ type: "LeaveMeeting" }));
  }

  sendOffer(target_user_id: string, offer: RTCSessionDescriptionInit) {
    this.ws.send(
      JSON.stringify({
        type: "SendOffer",
        offer,
        target_user_id,
      })
    );
  }

  sendAnswer(target_user_id: string, answer: RTCSessionDescriptionInit) {
    this.ws.send(
      JSON.stringify({
        type: "SendAnswer",
        target_user_id,
        answer,
      })
    );
  }

  sendIceCandidate(target_user_id: string, candidate: RTCIceCandidateInit) {
    this.ws.send(
      JSON.stringify({
        type: "SendIceCandidate",
        target_user_id,
        candidate: candidate,
      })
    );
  }

  async getCurrentMeeting(): Promise<MeetingData> {
    const res = await fetch(
      `http://localhost:3000/api/v1/meeting/${this.classId}`,
      {
        method: "GET",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
      }
    );
    const data = await res.json();
    return data;
  }
}
