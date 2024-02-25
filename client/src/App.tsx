import {EchoRequest, EchoServiceClient} from "./grpc-api/echo.ts";


function App() {

  return (
    <>
      Hey
        <button onClick={async () => {
          const echoService = new EchoServiceClient('http://localhost:9090');
          const req = new EchoRequest()
          req.message=("Message hey")
          const res = await echoService.Echo(req, null)
          console.log(res.message)
        }}>Click</button>
    </>
  )
}

export default App
