use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TelemetryGenericResponse<T> {
    pub status: bool,
    pub result: T,
}
