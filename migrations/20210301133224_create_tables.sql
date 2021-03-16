-- Create Subscriptions Table
CREATE TABLE subscriptions(
	   email TEXT NOT NULL UNIQUE,
	   name TEXT NOT NULL
);

-- Create Users Table
CREATE TABLE users(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
	   username TEXT NOT NULL UNIQUE,
       password TEXT
);

-- Create Carts Table
CREATE TABLE carts(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
	   user_id uuid NOT NULL,
	   FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Create products Table
CREATE TABLE products(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
	   name TEXT NOT NULL
);

-- Create Cart-Products Table
CREATE TABLE cart_products(
	   cart_id uuid NOT NULL,
       product_id uuid NOT NULL,
	   FOREIGN KEY (cart_id) REFERENCES carts (id),
       FOREIGN KEY (product_id) REFERENCES products (id)
);
