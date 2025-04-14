from open3d import *    

def main():
    cloud = io.read_point_cloud("output.ply") # Read point cloud
    open3d.visualization.draw_geometries([cloud])    # Visualize point cloud      

def draw_geometry_with_rotation(pcd):
    def rotate_view(vis):
        ctr = vis.get_view_control()
        ctr.rotate(10.0, 20.0)
        return False

    open3d.visualization.draw_geometries_with_animation_callback([pcd],
                                                              rotate_view)

if __name__ == "__main__":
    main()