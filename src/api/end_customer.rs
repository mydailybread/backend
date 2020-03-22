pub use crate::api::grpc::end_customer_server::{EndCustomer, EndCustomerServer};
use crate::api::grpc::{
    AvailableProductReply, AvailableProductRequest, Currency, CustomerInterestRequest,
    DeliveryPoint, DeliveryProduct, DeliveryStatus, MobileShop, OrderReply, OrderRequest,
    OrderStatusReply, OrderStatusRequest, OrderedProduct, Position, Product, Route,
};
use futures::channel::mpsc;
use futures_util::sink::SinkExt;
use prost_types::Timestamp;
use tonic::{Request, Response, Status, Streaming};

pub struct EndCustomerServerImpl {}

type RegisterCustomerInterestStream = mpsc::Receiver<Result<MobileShop, Status>>;

#[tonic::async_trait]
impl EndCustomer for EndCustomerServerImpl {
    type RegisterCustomerInterestStream = RegisterCustomerInterestStream;

    async fn register_customer_interest(
        &self,
        _request: Request<Streaming<CustomerInterestRequest>>,
    ) -> Result<Response<Self::RegisterCustomerInterestStream>, Status> {
        let route_points = vec![
            // Berlin, Schlossplatz
            Position {
                latitude: 52.518_230,
                longitude: 13.401_070,
            },
            // Berlin, Alexanderplatz
            Position {
                latitude: 52.521_751,
                longitude: 13.411_500,
            },
        ];

        let delivery_points = vec![
            DeliveryPoint {
                uuid: "3e7d0e06-9d65-4e79-afb8-e594e2162eca".to_string(),
                // position of this delivery point
                position: Some(Position {
                    latitude: 52.518_230,
                    longitude: 13.401_070,
                }),
                // planed arrival time for this delivery point
                scheduled_time: Some(Timestamp::default()),
                // minimum time in seconds delivery point is available at this position
                departure_time: Some(Timestamp::default()),
            },
            DeliveryPoint {
                uuid: "a9848bee-0ae2-4479-92e6-7c64657b860e".to_string(),
                // position of this delivery point
                position: Some(Position {
                    latitude: 52.521_751,
                    longitude: 13.411_500,
                }),
                // planed arrival time for this delivery point
                scheduled_time: Some(Timestamp::default()),
                // minimum time in seconds delivery point is available at this position
                departure_time: Some(Timestamp::default()),
            },
        ];

        let route = Route {
            route_uuid: "47b85ebc-9b72-490f-80e3-2e4e465a3853".to_string(),
            delivery_points,
            route_points,
        };

        let current_position = Position {
            latitude: 52.520_008,
            longitude: 13.404_954,
        };

        let current_delivery_point = DeliveryPoint {
            uuid: "3e7d0e06-9d65-4e79-afb8-e594e2162eca".to_string(),
            // position of this delivery point
            position: Some(Position {
                latitude: 52.518_230,
                longitude: 13.401_070,
            }),
            // planed arrival time for this delivery point
            scheduled_time: Some(Timestamp::default()),
            // minimum time in seconds delivery point is available at this position
            departure_time: Some(Timestamp::default()),
        };

        let next_delivery_point = DeliveryPoint {
            uuid: "a9848bee-0ae2-4479-92e6-7c64657b860e".to_string(),
            // position of this delivery point
            position: Some(Position {
                latitude: 52.521_751,
                longitude: 13.411_500,
            }),
            // planed arrival time for this delivery point
            scheduled_time: Some(Timestamp::default()),
            // minimum time in seconds delivery point is available at this position
            departure_time: Some(Timestamp::default()),
        };

        let mobile_shop = MobileShop {
            mobile_shop_uuid: "e6d99988-a0ed-4665-a368-be1847146c2b".to_string(),
            mobile_url: "https://lieferemma.de".to_string(),
            // Title of delivery to be displayed to customer e.g. Bakery John Doe
            title: "Bäckerei Max Musterfrau".to_string(),
            // Last location updated
            current_position: Some(current_position),
            // Last location update
            last_seen: Some(Timestamp::default()),
            // Current delivery point
            current_delivery_point: Some(current_delivery_point),
            // Next delivery point
            next_delivery_point: Some(next_delivery_point),
            // Is the delivery vehicle currently stationary or not
            delivery_status: DeliveryStatus::Parked as i32,
            // Estimated time of arrival at next delivery point in seconds
            next_delivery_point_eta: 0,
            // Estimated time of arrival at the pick up delivery point in seconds
            pick_up_delivery_point_eta: 0,
            route: Some(route),
        };

        let (mut tx, rx) = mpsc::channel(4);
        let shops = vec![mobile_shop];
        tokio::spawn(async move {
            for shop in &shops[..] {
                tx.send(Ok(shop.clone())).await.unwrap();
            }
        });

        Ok(Response::new(rx))
    }

    async fn place_order(
        &self,
        _request: Request<OrderRequest>,
    ) -> Result<Response<OrderReply>, Status> {
        let product = Product {
            product_uiid: "91ea969e-6cd8-41ab-8faa-636cb9ffd991".to_string(),
            title: "Kaisersemmel".to_string(),
            description: "Unser Klassiker, das Kaiserbrötchen. Macht sich immer gut entweder mit Nutella oder Marmelade.".to_string(),
            url: "https://upload.wikimedia.org/wikipedia/commons/d/d0/Kaisersemmel-.jpg".to_string(),
            price: 90,
            currency: Currency::Eur as i32,
        };

        let ordered_product = OrderedProduct {
            product: Some(product),
            quantity_ordered: 5,
            total_price: 450,
            currency: Currency::Eur as i32,
        };

        let pick_up_point = DeliveryPoint {
            uuid: "a9848bee-0ae2-4479-92e6-7c64657b860e".to_string(),
            // position of this delivery point
            position: Some(Position {
                latitude: 52.521_751,
                longitude: 13.411_500,
            }),
            // planed arrival time for this delivery point
            scheduled_time: Some(Timestamp::default()),
            // minimum time in seconds delivery point is available at this position
            departure_time: Some(Timestamp::default()),
        };

        let order_reply = OrderReply {
            order_uuid: "3feedb57-9f6e-476f-93fb-5515ea831d5f".to_string(),
            order_id: "abcd".to_string(),
            pick_up_point: Some(pick_up_point),
            currency: Currency::Eur as i32,
            total: 450,
            ordered_products: vec![ordered_product],
        };

        Ok(Response::new(order_reply))
    }

    async fn available_products(
        &self,
        _request: Request<AvailableProductRequest>,
    ) -> Result<Response<AvailableProductReply>, Status> {
        let product = Product {
            product_uiid: "91ea969e-6cd8-41ab-8faa-636cb9ffd991".to_string(),
            title: "Kaisersemmel".to_string(),
            description: "Unser Klassiker, das Kaiserbrötchen. Macht sich immer gut entweder mit Nutella oder Marmelade.".to_string(),
            url: "https://upload.wikimedia.org/wikipedia/commons/d/d0/Kaisersemmel-.jpg".to_string(),
            price: 90,
            currency: Currency::Eur as i32,
        };

        let deliverable_product = DeliveryProduct {
            product: Some(product),
            quantity_available: 100,
        };

        let reply = AvailableProductReply {
            updated: Some(Timestamp::default()),
            deliverable_products: vec![deliverable_product],
        };

        Ok(Response::new(reply))
    }

    async fn order_status(
        &self,
        _request: Request<OrderStatusRequest>,
    ) -> Result<Response<OrderStatusReply>, Status> {
        unimplemented!()
    }
}
