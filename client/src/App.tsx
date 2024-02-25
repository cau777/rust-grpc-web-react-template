import {EchoRequest, EchoServiceClient} from "./grpc-api/echo.ts";


function App() {

  return (
    <>
      Hey
        <button onClick={async () => {
          const echoService = new EchoServiceClient('http://localhost:50051', undefined, {
            // suppressCorsPreflight: true
          });
          const req = new EchoRequest()
          req.message=("Message hey")
          const res = await echoService.Echo(req, {})
          console.log(res.message)
        }}>Click</button>
    </>
  )
}

export default App
