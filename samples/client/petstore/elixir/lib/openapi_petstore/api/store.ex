# NOTE: This file is auto generated by OpenAPI Generator 7.11.0-SNAPSHOT (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule OpenapiPetstore.Api.Store do
  @moduledoc """
  API calls for all endpoints tagged `Store`.
  """

  alias OpenapiPetstore.Connection
  import OpenapiPetstore.RequestBuilder

  @doc """
  Delete purchase order by ID
  For valid response try integer IDs with value < 1000. Anything above 1000 or nonintegers will generate API errors

  ### Parameters

  - `connection` (OpenapiPetstore.Connection): Connection to server
  - `order_id` (String.t): ID of the order that needs to be deleted
  - `opts` (keyword): Optional parameters

  ### Returns

  - `{:ok, nil}` on success
  - `{:error, Tesla.Env.t}` on failure
  """
  @spec delete_order(Tesla.Env.client, String.t, keyword()) :: {:ok, nil} | {:error, Tesla.Env.t}
  def delete_order(connection, order_id, _opts \\ []) do
    request =
      %{}
      |> method(:delete)
      |> url("/store/order/#{order_id}")
      |> Enum.into([])

    connection
    |> Connection.request(request)
    |> evaluate_response([
      {400, false},
      {404, false}
    ])
  end

  @doc """
  Returns pet inventories by status
  Returns a map of status codes to quantities

  ### Parameters

  - `connection` (OpenapiPetstore.Connection): Connection to server
  - `opts` (keyword): Optional parameters

  ### Returns

  - `{:ok, %{}}` on success
  - `{:error, Tesla.Env.t}` on failure
  """
  @spec get_inventory(Tesla.Env.client, keyword()) :: {:ok, map()} | {:error, Tesla.Env.t}
  def get_inventory(connection, _opts \\ []) do
    request =
      %{}
      |> method(:get)
      |> url("/store/inventory")
      |> Enum.into([])

    connection
    |> Connection.request(request)
    |> evaluate_response([
      {200, %{}}
    ])
  end

  @doc """
  Find purchase order by ID
  For valid response try integer IDs with value <= 5 or > 10. Other values will generate exceptions

  ### Parameters

  - `connection` (OpenapiPetstore.Connection): Connection to server
  - `order_id` (integer()): ID of pet that needs to be fetched
  - `opts` (keyword): Optional parameters

  ### Returns

  - `{:ok, OpenapiPetstore.Model.Order.t}` on success
  - `{:error, Tesla.Env.t}` on failure
  """
  @spec get_order_by_id(Tesla.Env.client, integer(), keyword()) :: {:ok, nil} | {:ok, OpenapiPetstore.Model.Order.t} | {:error, Tesla.Env.t}
  def get_order_by_id(connection, order_id, _opts \\ []) do
    request =
      %{}
      |> method(:get)
      |> url("/store/order/#{order_id}")
      |> Enum.into([])

    connection
    |> Connection.request(request)
    |> evaluate_response([
      {200, OpenapiPetstore.Model.Order},
      {400, false},
      {404, false}
    ])
  end

  @doc """
  Place an order for a pet
  

  ### Parameters

  - `connection` (OpenapiPetstore.Connection): Connection to server
  - `order` (Order): order placed for purchasing the pet
  - `opts` (keyword): Optional parameters

  ### Returns

  - `{:ok, OpenapiPetstore.Model.Order.t}` on success
  - `{:error, Tesla.Env.t}` on failure
  """
  @spec place_order(Tesla.Env.client, OpenapiPetstore.Model.Order.t, keyword()) :: {:ok, nil} | {:ok, OpenapiPetstore.Model.Order.t} | {:error, Tesla.Env.t}
  def place_order(connection, order, _opts \\ []) do
    request =
      %{}
      |> method(:post)
      |> url("/store/order")
      |> add_param(:body, :body, order)
      |> Enum.into([])

    connection
    |> Connection.request(request)
    |> evaluate_response([
      {200, OpenapiPetstore.Model.Order},
      {400, false}
    ])
  end
end
