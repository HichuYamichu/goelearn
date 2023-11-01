type SignalServerEventData =
  | MeetingStartedData
  | UserJoinedData
  | OfferData
  | AnswerData
  | IceCandidateData;

type MeetingStartedData = { type: "MeetingStarted" };
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

type SignalServerMessage = {};

export class ClassMeetingWS {
  ws: WebSocket;

  onMeetingStarted?: (data: MeetingStartedData) => Promise<void>;
  onUserJoined?: (data: UserJoinedData) => Promise<void>;
  onOffer?: (data: OfferData) => Promise<void>;
  onAnswer?: (data: AnswerData) => Promise<void>;
  onICECandidate?: (data: IceCandidateData) => Promise<void>;

  constructor(classId: string) {
    this.ws = new WebSocket("ws://localhost:3000/rtc-ws");
    const token = localStorage.getItem("token");

    this.ws.addEventListener("open", async (event) => {
      this.ws.send(JSON.stringify({ type: "Auth", token }));
      this.ws.send(
        JSON.stringify({ type: "Subscribe", target_class_id: classId })
      );
    });

    this.ws.addEventListener("message", async (event) => {
      const data: SignalServerEventData = JSON.parse(event.data);
      if (data.type === "MeetingStarted") {
        await this.onMeetingStarted?.(data);
      } else if (data.type === "UserJoined") {
        await this.onUserJoined?.(data);
      } else if (data.type === "Offer") {
        await this.onOffer?.(data);
      } else if (data.type === "Answer") {
        await this.onAnswer?.(data);
      } else if (data.type === "IceCandidate") {
        await this.onICECandidate?.(data);
      }
    });
  }

  startMeeting(target_class_id: string) {
    this.ws.send(JSON.stringify({ type: "StartMeeting", target_class_id }));
  }

  joinMeeting(target_class_id: string) {
    this.ws.send(JSON.stringify({ type: "JoinMeeting", target_class_id }));
  }

  sendOffer(
    target_user_id: string,
    target_class_id: string,
    offer: RTCSessionDescriptionInit
  ) {
    this.ws.send(
      JSON.stringify({
        type: "SendOffer",
        offer,
        target_class_id,
        target_user_id,
      })
    );
  }

  sendAnswer(
    target_user_id: string,
    target_class_id: string,
    answer: RTCSessionDescriptionInit
  ) {
    this.ws.send(
      JSON.stringify({
        type: "SendAnswer",
        target_class_id,
        target_user_id,
        answer,
      })
    );
  }

  sendIceCandidate(
    target_user_id: string,
    target_class_id: string,
    candidate: RTCIceCandidateInit
  ) {
    this.ws.send(
      JSON.stringify({
        type: "SendIceCandidate",
        target_class_id,
        target_user_id,
        candidate: candidate,
      })
    );
  }
}
