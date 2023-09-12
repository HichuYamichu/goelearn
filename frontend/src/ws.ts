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

export class RTCWS {
  ws: WebSocket;

  meetingStartedHandler?: (data: MeetingStartedData) => Promise<void>;
  userJoinedHandler?: (data: UserJoinedData) => Promise<void>;
  offerHandler?: (data: OfferData) => Promise<void>;
  answerHandler?: (data: AnswerData) => Promise<void>;
  iceCandidateHandler?: (data: IceCandidateData) => Promise<void>;

  constructor(url: string) {
    this.ws = new WebSocket(url);
    this.ws.addEventListener("message", async (event) => {
      const data: SignalServerEventData = JSON.parse(event.data);
      if (data.type === "MeetingStarted") {
        await this.meetingStartedHandler?.(data);
      } else if (data.type === "UserJoined") {
        await this.userJoinedHandler?.(data);
      } else if (data.type === "Offer") {
        await this.offerHandler?.(data);
      } else if (data.type === "Answer") {
        await this.answerHandler?.(data);
      } else if (data.type === "IceCandidate") {
        await this.iceCandidateHandler?.(data);
      }
    });
  }

  auth(token: string) {
    this.ws.addEventListener("open", (event) => {
      this.ws.send(JSON.stringify({ type: "Auth", token }));
    });
  }

  subscribe(target_class_id: string) {
    this.ws.addEventListener("open", (event) => {
      this.ws.send(JSON.stringify({ type: "Subscribe", target_class_id }));
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
