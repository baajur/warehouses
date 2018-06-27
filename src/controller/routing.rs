use models::*;

use stq_router::*;

#[derive(Clone, Debug)]
pub enum Route {
    Warehouses,
    WarehousesByStore {
        store_id: StoreId,
    },
    Warehouse {
        warehouse_id: WarehouseIdentifier,
    },
    StocksInWarehouse {
        warehouse_id: WarehouseId,
    },
    StockInWarehouse {
        warehouse_id: WarehouseId,
        product_id: ProductId,
    },
    StocksByProductId {
        product_id: ProductId,
    },
    StockById {
        warehouse_product_id: StockId,
    },
    Roles,
    RoleById {
        role_id: RoleId,
    },
    RolesByUserId {
        user_id: UserId,
    },
}

pub fn make_router() -> RouteParser<Route> {
    let mut route_parser: RouteParser<Route> = Default::default();

    route_parser.add_route(r"^/warehouses$", || Route::Warehouses);
    route_parser.add_route_with_params(r"^/warehouses/by-id/(\S+)/products$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|warehouse_id| Route::StocksInWarehouse { warehouse_id })
    });
    route_parser.add_route_with_params(r"^/warehouses/by-id/(\S+)/products/(\d+)$", |params| {
        if let Some(warehouse_id_s) = params.get(0) {
            if let Some(product_id_s) = params.get(1) {
                if let Ok(warehouse_id) = warehouse_id_s.parse() {
                    let warehouse_id = WarehouseId(warehouse_id);
                    if let Ok(product_id) = product_id_s.parse() {
                        let product_id = ProductId(product_id);
                        return Some(Route::StockInWarehouse {
                            warehouse_id,
                            product_id,
                        });
                    }
                }
            }
        }
        None
    });
    route_parser.add_route_with_params(r"^/warehouses/by-id/(\S+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok().map(WarehouseIdentifier::Id))
            .map(|warehouse_id| Route::Warehouse { warehouse_id })
    });
    route_parser.add_route_with_params(r"^/warehouses/by-slug/(\S+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok().map(WarehouseIdentifier::Slug))
            .map(|warehouse_id| Route::Warehouse { warehouse_id })
    });
    route_parser.add_route_with_params(r"^/warehouses/by-store-id/(\d+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|store_id| Route::WarehousesByStore { store_id })
    });

    route_parser.add_route_with_params(r"^/stocks/by-product-id/(\d+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|product_id| Route::StocksByProductId { product_id })
    });
    route_parser.add_route_with_params(r"^/stocks/by-id/(\S+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|warehouse_product_id| Route::StockById {
                warehouse_product_id,
            })
    });

    route_parser.add_route(r"^/roles$", || Route::Roles);
    route_parser.add_route_with_params(r"^/roles/by-user-id/(\d+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|user_id| Route::RolesByUserId { user_id })
    });
    route_parser.add_route_with_params(r"^/roles/by-id/(\S+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|role_id| Route::RoleById { role_id })
    });

    route_parser
}
