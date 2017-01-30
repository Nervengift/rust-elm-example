import Html exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Decode

main : Program Never Model Msg
main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


-- MODEL

type alias Model =
  { state : Bool
  , err : Maybe Http.Error
  }


-- UPDATE

type Msg =
    Toggle
    | UpdateValue (Result Http.Error Bool)

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Toggle ->
            (model, toggleState)

        UpdateValue (Ok newState) ->
            ({model|state = newState, err = Nothing}, Cmd.none)

        UpdateValue (Err e) ->
            ({model|err = Just e}, Cmd.none)

toggleState : Cmd Msg
toggleState =
    let
        url =
            "/api/toggle"
        request =
            Http.get url decodeStatus
    in
       Http.send UpdateValue request

decodeStatus : Decode.Decoder Bool
decodeStatus =
    Decode.at ["status"] Decode.bool


-- VIEW

view : Model -> Html Msg
view model =
    div []
        [ h1 [] [ text (toString model.state) ]
        , button [ onClick Toggle ] [ text "toggle" ]
        , div [] [ (showErr model.err) ]
        ]

showErr : Maybe Http.Error -> Html Msg
showErr err = 
    case err of
        Just e ->
            text (toString e)
        Nothing ->
            text ("")


-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none


-- INIT

init : (Model, Cmd Msg)
init =
  (Model False Nothing, toggleState)
