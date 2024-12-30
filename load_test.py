import aiohttp
import asyncio
import time

# Define the URL for the request
url = 'http://localhost:5001/build_metadata?version=1.0.3&branch=master&image_name=imagge'

# Function to send a single request and measure the response time
async def send_request(session, url, total_time, count):
    try:
        start_time = time.time()  # Start time
        async with session.get(url) as response:
            status = response.status
            text = await response.text()
            end_time = time.time()  # End time
            response_time = end_time - start_time
            total_time += response_time  # Accumulate total time
            count += 1  # Increment count
    except Exception as e:
        print(f"Request failed: {e}")
    return total_time, count

# Function to send requests in batches and calculate average response time
async def send_requests(url, initial_rate=1000, rate_increment=1000, duration=60, total_duration=600):
    total_time = 0
    count = 0
    requests_per_second = initial_rate  # Initial requests per second
    start_time = time.time()  # Start the overall timer

    async with aiohttp.ClientSession() as session:
        # Loop for every minute, incrementing the request rate
        while True:
            # Check if 10 minutes have passed
            elapsed_time = time.time() - start_time
            if elapsed_time >= total_duration:
                print("Simulation finished after 10 minutes.")
                break  # Stop the loop after 10 minutes

            tasks = []
            for _ in range(requests_per_second):
                tasks.append(send_request(session, url, total_time, count))

            # Execute all tasks concurrently
            results = await asyncio.gather(*tasks)

            # Calculate total time and count from the results
            total_time = sum(result[0] for result in results)
            count = sum(result[1] for result in results)

            # Calculate and print average response time for the batch
            if count > 0:
                average_response_time = total_time / count
                print(f"Sent {requests_per_second} requests, Average Response Time: {average_response_time:.4f}s")
            else:
                print("No requests were completed successfully.")

            # Increase the rate after one minute
            await asyncio.sleep(duration)  # Sleep for the duration of the current minute
            requests_per_second += rate_increment  # Increase the request rate

# Main entry point
if __name__ == "__main__":
    asyncio.run(send_requests(url, initial_rate=1000, rate_increment=1000, duration=10, total_duration=600))
